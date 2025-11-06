#![no_main]

use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct SteganographyInput {
    carrier: Vec<u8>,
    secret_data: Vec<u8>,
}

fuzz_target!(|input: SteganographyInput| {
    // Fuzz steganography operations
    // Test for:
    // - Data recovery
    // - Capacity limits
    // - Carrier corruption

    // Limit sizes to prevent timeout
    if input.carrier.len() > 10000 || input.secret_data.len() > 1000 {
        return;
    }

    if input.carrier.is_empty() || input.secret_data.is_empty() {
        return;
    }

    // Test hiding data
    if let Ok(stego_carrier) = hide_data(&input.carrier, &input.secret_data) {
        // Verify properties:
        // 1. Carrier size doesn't change drastically
        assert!(stego_carrier.len() <= input.carrier.len() * 2);
        assert!(stego_carrier.len() >= input.carrier.len());

        // 2. Can extract hidden data
        if let Ok(extracted) = extract_data(&stego_carrier) {
            // 3. Extracted data matches original
            assert_eq!(extracted, input.secret_data);
        }

        // 4. Steganographic capacity limit
        let capacity = calculate_capacity(&input.carrier);
        if input.secret_data.len() > capacity {
            // Should fail to hide data exceeding capacity
            let oversized = vec![0u8; capacity + 100];
            assert!(hide_data(&input.carrier, &oversized).is_err());
        }
    }

    // Test extraction from unmodified carrier (should fail gracefully)
    let _ = extract_data(&input.carrier);
});

fn hide_data(carrier: &[u8], secret: &[u8]) -> Result<Vec<u8>, ()> {
    // Check capacity
    let capacity = calculate_capacity(carrier);
    if secret.len() > capacity {
        return Err(());
    }

    // Simple LSB steganography
    let mut stego = carrier.to_vec();

    // Reserve first 4 bytes for secret length
    if carrier.len() < 32 {
        return Err(());
    }

    let secret_len = secret.len() as u32;
    let len_bytes = secret_len.to_le_bytes();

    // Hide length in first 32 bits
    for (i, &len_byte) in len_bytes.iter().enumerate() {
        for bit in 0..8 {
            let bit_val = (len_byte >> bit) & 1;
            let carrier_idx = i * 8 + bit;
            if carrier_idx >= stego.len() {
                return Err(());
            }
            stego[carrier_idx] = (stego[carrier_idx] & 0xFE) | bit_val;
        }
    }

    // Hide secret data
    let start_offset = 32;
    for (i, &secret_byte) in secret.iter().enumerate() {
        for bit in 0..8 {
            let bit_val = (secret_byte >> bit) & 1;
            let carrier_idx = start_offset + i * 8 + bit;
            if carrier_idx >= stego.len() {
                return Err(());
            }
            stego[carrier_idx] = (stego[carrier_idx] & 0xFE) | bit_val;
        }
    }

    Ok(stego)
}

fn extract_data(stego: &[u8]) -> Result<Vec<u8>, ()> {
    if stego.len() < 32 {
        return Err(());
    }

    // Extract length from first 32 bits
    let mut len_bytes = [0u8; 4];
    for (i, len_byte) in len_bytes.iter_mut().enumerate() {
        for bit in 0..8 {
            let carrier_idx = i * 8 + bit;
            let bit_val = stego[carrier_idx] & 1;
            *len_byte |= bit_val << bit;
        }
    }

    let secret_len = u32::from_le_bytes(len_bytes) as usize;

    // Sanity check
    if secret_len > stego.len() || secret_len > 10000 {
        return Err(());
    }

    // Extract secret data
    let mut secret = Vec::new();
    let start_offset = 32;

    for i in 0..secret_len {
        let mut secret_byte = 0u8;
        for bit in 0..8 {
            let carrier_idx = start_offset + i * 8 + bit;
            if carrier_idx >= stego.len() {
                return Err(());
            }
            let bit_val = stego[carrier_idx] & 1;
            secret_byte |= bit_val << bit;
        }
        secret.push(secret_byte);
    }

    Ok(secret)
}

fn calculate_capacity(carrier: &[u8]) -> usize {
    // LSB steganography: 1 bit per byte, minus overhead for length
    if carrier.len() < 32 {
        return 0;
    }
    (carrier.len() - 32) / 8
}
