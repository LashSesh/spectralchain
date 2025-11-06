/*!
 * Performance Benchmarks - Ghost Protocol
 *
 * Benchmarks for the complete Ghost Protocol flow and individual components.
 */

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use mef_ghost_network::packet::{CarrierType, ResonanceState};
use mef_ghost_network::protocol::{GhostProtocol, MaskingParams, ProtocolConfig};
use mef_ghost_network::transport::{PacketCodec, WireFormat};

// Benchmark complete 6-step protocol flow
fn bench_complete_protocol_flow(c: &mut Criterion) {
    let protocol = GhostProtocol::default();
    let sender_resonance = ResonanceState::new(1.0, 1.0, 1.0);
    let target_resonance = ResonanceState::new(2.0, 2.0, 2.0);
    let action = vec![0u8; 256]; // 256 bytes payload

    c.bench_function("protocol_6_steps_complete", |b| {
        b.iter(|| {
            // Step 1: Create transaction
            let tx = protocol
                .create_transaction(
                    black_box(sender_resonance),
                    black_box(target_resonance),
                    black_box(action.clone()),
                )
                .unwrap();

            // Step 2: Mask transaction
            let params = MaskingParams::from_resonance(&sender_resonance, &target_resonance);
            let masked = protocol.mask_transaction(&tx, &params).unwrap();

            // Step 3: Embed in carrier
            let carrier = protocol
                .embed_transaction(&masked, CarrierType::Raw)
                .unwrap();

            // Step 4: Create packet
            let packet = protocol
                .create_packet(&tx, masked, carrier, CarrierType::Raw, &params)
                .unwrap();

            // Step 5-6: Reception would happen on receiver side
            black_box(packet)
        })
    });
}

// Benchmark individual protocol steps
fn bench_protocol_steps(c: &mut Criterion) {
    let protocol = GhostProtocol::default();
    let sender_resonance = ResonanceState::new(1.0, 1.0, 1.0);
    let target_resonance = ResonanceState::new(2.0, 2.0, 2.0);
    let action = vec![0u8; 256];

    // Step 1: Transaction creation
    c.bench_function("protocol_step1_create_transaction", |b| {
        b.iter(|| {
            protocol
                .create_transaction(
                    black_box(sender_resonance),
                    black_box(target_resonance),
                    black_box(action.clone()),
                )
                .unwrap()
        })
    });

    // Step 2: Masking
    let tx = protocol
        .create_transaction(sender_resonance, target_resonance, action.clone())
        .unwrap();
    let params = MaskingParams::from_resonance(&sender_resonance, &target_resonance);

    c.bench_function("protocol_step2_mask_transaction", |b| {
        b.iter(|| {
            protocol
                .mask_transaction(black_box(&tx), black_box(&params))
                .unwrap()
        })
    });

    // Step 3: Steganography
    let masked = protocol.mask_transaction(&tx, &params).unwrap();

    c.bench_function("protocol_step3_embed_transaction", |b| {
        b.iter(|| {
            protocol
                .embed_transaction(black_box(&masked), CarrierType::Raw)
                .unwrap()
        })
    });
}

// Benchmark packet codec (serialization/deserialization)
fn bench_packet_codec(c: &mut Criterion) {
    let mut group = c.benchmark_group("packet_codec");

    let protocol = GhostProtocol::default();
    let resonance = ResonanceState::new(1.0, 1.0, 1.0);
    let tx = protocol
        .create_transaction(resonance, resonance, vec![0u8; 256])
        .unwrap();
    let params = MaskingParams::from_resonance(&resonance, &resonance);
    let masked = protocol.mask_transaction(&tx, &params).unwrap();
    let carrier = protocol
        .embed_transaction(&masked, CarrierType::Raw)
        .unwrap();
    let packet = protocol
        .create_packet(&tx, masked, carrier, CarrierType::Raw, &params)
        .unwrap();

    // Bincode codec
    let codec_bincode = PacketCodec::new(WireFormat::Bincode);
    group.bench_function("encode_bincode", |b| {
        b.iter(|| codec_bincode.encode(black_box(&packet)).unwrap())
    });

    let encoded_bincode = codec_bincode.encode(&packet).unwrap();
    group.bench_function("decode_bincode", |b| {
        b.iter(|| codec_bincode.decode(black_box(&encoded_bincode)).unwrap())
    });

    // JSON codec
    let codec_json = PacketCodec::new(WireFormat::Json);
    group.bench_function("encode_json", |b| {
        b.iter(|| codec_json.encode(black_box(&packet)).unwrap())
    });

    let encoded_json = codec_json.encode(&packet).unwrap();
    group.bench_function("decode_json", |b| {
        b.iter(|| codec_json.decode(black_box(&encoded_json)).unwrap())
    });

    group.finish();
}

// Benchmark resonance matching
fn bench_resonance_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("resonance_matching");

    let node_resonance = ResonanceState::new(1.0, 1.0, 1.0);
    let packet_resonance = ResonanceState::new(1.05, 1.05, 1.05);
    let epsilon = 0.1;

    group.bench_function("resonance_match_close", |b| {
        b.iter(|| {
            packet_resonance.matches_resonance(black_box(&node_resonance), black_box(epsilon))
        })
    });

    let far_resonance = ResonanceState::new(10.0, 10.0, 10.0);

    group.bench_function("resonance_match_far", |b| {
        b.iter(|| far_resonance.matches_resonance(black_box(&node_resonance), black_box(epsilon)))
    });

    group.finish();
}

// Benchmark masking parameter derivation
fn bench_masking_params(c: &mut Criterion) {
    let sender_resonance = ResonanceState::new(1.0, 1.0, 1.0);
    let target_resonance = ResonanceState::new(2.0, 2.0, 2.0);

    c.bench_function("masking_params_from_resonance", |b| {
        b.iter(|| {
            MaskingParams::from_resonance(
                black_box(&sender_resonance),
                black_box(&target_resonance),
            )
        })
    });

    c.bench_function("masking_params_ephemeral_key", |b| {
        b.iter(|| MaskingParams::generate_ephemeral_key())
    });
}

// Benchmark with varying payload sizes
fn bench_protocol_payload_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("protocol_payload_sizes");

    let protocol = GhostProtocol::default();
    let resonance = ResonanceState::new(1.0, 1.0, 1.0);
    let params = MaskingParams::from_resonance(&resonance, &resonance);

    for size in [64, 256, 1024, 4096, 16384].iter() {
        let action = vec![0u8; *size];

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let tx = protocol
                    .create_transaction(resonance, resonance, action.clone())
                    .unwrap();
                let masked = protocol.mask_transaction(&tx, &params).unwrap();
                black_box(masked)
            })
        });
    }

    group.finish();
}

// Benchmark key rotation
fn bench_key_rotation(c: &mut Criterion) {
    let sender_resonance = ResonanceState::new(1.0, 1.0, 1.0);
    let target_resonance = ResonanceState::new(2.0, 2.0, 2.0);

    c.bench_function("key_rotation_epoch_derivation", |b| {
        b.iter(|| {
            // Key rotation derives params based on current epoch
            let params = MaskingParams::from_resonance(
                black_box(&sender_resonance),
                black_box(&target_resonance),
            );
            black_box(params)
        })
    });
}

// Benchmark ZK proof operations
fn bench_zk_proofs(c: &mut Criterion) {
    let protocol = GhostProtocol::default();
    let sender_resonance = ResonanceState::new(1.0, 1.0, 1.0);
    let target_resonance = ResonanceState::new(2.0, 2.0, 2.0);
    let action = vec![0u8; 256];

    // ZK proof generation happens in create_transaction
    c.bench_function("zk_proof_generation", |b| {
        b.iter(|| {
            protocol
                .create_transaction(
                    black_box(sender_resonance),
                    black_box(target_resonance),
                    black_box(action.clone()),
                )
                .unwrap()
        })
    });

    // ZK proof verification happens in receive_packet
    let tx = protocol
        .create_transaction(sender_resonance, target_resonance, action.clone())
        .unwrap();
    let params = MaskingParams::from_resonance(&sender_resonance, &target_resonance);
    let masked = protocol.mask_transaction(&tx, &params).unwrap();
    let carrier = protocol
        .embed_transaction(&masked, CarrierType::Raw)
        .unwrap();
    let packet = protocol
        .create_packet(&tx, masked, carrier, CarrierType::Raw, &params)
        .unwrap();

    c.bench_function("zk_proof_verification", |b| {
        b.iter(|| {
            protocol
                .receive_packet(black_box(&packet), black_box(&target_resonance))
                .unwrap()
        })
    });
}

criterion_group!(
    benches,
    bench_complete_protocol_flow,
    bench_protocol_steps,
    bench_packet_codec,
    bench_resonance_matching,
    bench_masking_params,
    bench_protocol_payload_sizes,
    bench_key_rotation,
    bench_zk_proofs,
);

criterion_main!(benches);
