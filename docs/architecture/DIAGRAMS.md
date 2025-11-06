# SpectralChain Architecture Diagrams

**Version**: 2.0.0
**Last Updated**: 2025-11-06

This document contains visual architecture diagrams using Mermaid notation.

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Component Architecture](#component-architecture)
3. [Data Flow](#data-flow)
4. [API Architecture](#api-architecture)
5. [CLI Architecture](#cli-architecture)
6. [Processing Pipeline](#processing-pipeline)
7. [Ledger Architecture](#ledger-architecture)
8. [Ghost Network](#ghost-network)
9. [Quantum Operators](#quantum-operators)
10. [Deployment Architectures](#deployment-architectures)

---

## System Overview

### High-Level Architecture

```mermaid
graph TB
    subgraph "Client Layer"
        CLI[CLI mef]
        API_CLIENT[API Client]
        SDK[Rust SDK]
        GUI[GUI - Future]
    end

    subgraph "API Layer"
        REST[REST API - Axum]
        AUTH[Authentication]
        ROUTER[Router]
    end

    subgraph "Core Processing"
        MEF[MEF-Core Pipeline]
        SPIRAL[Spiral Snapshots]
        SOLVE[Solve-Coagula]
        TIC[TIC Crystallizer]
    end

    subgraph "Quantum Extensions"
        GHOST[Ghost Network]
        QOP[Quantum Operators]
        EPHEMERAL[Ephemeral Services]
        FORK[Fork Healing]
    end

    subgraph "Storage Layer"
        LEDGER[Infinity Ledger]
        VECTOR_DB[Vector Database]
        STORAGE[File Storage]
    end

    subgraph "Monitoring"
        METRICS[Prometheus Metrics]
        LOGS[Logging]
        AUDIT[Audit Trail]
    end

    CLI --> REST
    API_CLIENT --> REST
    SDK --> MEF
    GUI -.-> REST

    REST --> AUTH
    AUTH --> ROUTER
    ROUTER --> MEF

    MEF --> SPIRAL
    SPIRAL --> SOLVE
    SOLVE --> TIC

    MEF --> GHOST
    MEF --> QOP
    MEF --> EPHEMERAL
    MEF --> FORK

    TIC --> LEDGER
    MEF --> VECTOR_DB
    SPIRAL --> STORAGE

    REST --> METRICS
    MEF --> LOGS
    LEDGER --> AUDIT
```

---

## Component Architecture

### Core Module Dependencies

```mermaid
graph LR
    subgraph "Core Modules"
        CORE[mef-core]
        SPIRAL[mef-spiral]
        LEDGER[mef-ledger]
        HDAG[mef-hdag]
        TIC[mef-tic]
        COUPLING[mef-coupling]
        TOPOLOGY[mef-topology]
        DOMAINS[mef-domains]
        VECTOR[mef-vector-db]
        SOLVE[mef-solvecoagula]
        AUDIT[mef-audit]
    end

    subgraph "Extension Modules"
        KNOWLEDGE[mef-knowledge]
        MEMORY[mef-memory]
        ROUTER[mef-router]
        SCHEMAS[mef-schemas]
    end

    subgraph "Quantum Modules"
        QOPS[mef-quantum-ops]
        GHOST[mef-ghost-network]
        EPHEMERAL[mef-ephemeral-services]
        FORK[mef-fork-healing]
        QROUTE[mef-quantum-routing]
    end

    subgraph "Applications"
        API[mef-api]
        CLI[mef-cli]
    end

    CORE --> SPIRAL
    CORE --> HDAG
    SPIRAL --> SOLVE
    SOLVE --> TIC
    TIC --> LEDGER
    CORE --> COUPLING
    CORE --> TOPOLOGY
    CORE --> DOMAINS
    DOMAINS --> VECTOR
    LEDGER --> AUDIT

    KNOWLEDGE --> MEMORY
    KNOWLEDGE --> ROUTER
    MEMORY --> SCHEMAS
    ROUTER --> SCHEMAS

    GHOST --> QOPS
    EPHEMERAL --> GHOST
    FORK --> CORE
    QROUTE --> GHOST

    API --> CORE
    API --> KNOWLEDGE
    API --> GHOST
    CLI --> CORE
```

---

## Data Flow

### Ingest → Process → Ledger Flow

```mermaid
sequenceDiagram
    participant Client
    participant API
    participant MEF as MEF-Core
    participant Spiral
    participant Solve as Solve-Coagula
    participant TIC
    participant Ledger

    Client->>API: POST /ingest {data}
    API->>MEF: Normalize data
    MEF->>Spiral: Create snapshot
    Spiral->>Spiral: Generate PoR
    Spiral-->>API: snapshot_id + PoR
    API-->>Client: {snapshot_id, por}

    Client->>API: POST /process {snapshot_id}
    API->>Spiral: Load snapshot
    Spiral->>Solve: Process snapshot
    Solve->>Solve: XSwap iterations
    Solve->>Solve: Eigenvalue convergence
    Solve->>TIC: Crystallize TIC
    TIC->>Ledger: Append block
    Ledger->>Ledger: Update hash chain
    TIC-->>API: {tic_id, converged}
    API-->>Client: {tic_id, iterations}
```

### Vector Search Flow

```mermaid
sequenceDiagram
    participant Client
    participant API
    participant IndexMgr as Index Manager
    participant Provider as Vector Provider
    participant HNSW
    participant Storage

    Client->>API: POST /search
    API->>IndexMgr: Search request
    IndexMgr->>Provider: Get provider (HNSW)
    Provider->>HNSW: ANN search
    HNSW->>HNSW: Navigate graph
    HNSW->>Storage: Fetch vectors
    Storage-->>HNSW: Vector data
    HNSW-->>Provider: Top-k results
    Provider-->>IndexMgr: Ranked results
    IndexMgr-->>API: Search results
    API-->>Client: {results, took_ms}
```

---

## API Architecture

### REST API Structure

```mermaid
graph TB
    subgraph "API Server"
        ENTRY[main.rs]
        CONFIG[Configuration]
        SERVER[Axum Server]

        subgraph "Middleware"
            AUTH_MW[Auth Middleware]
            LOGGING[Logging]
            CORS[CORS]
            METRICS_MW[Metrics]
        end

        subgraph "Routes"
            HEALTH[/health]
            INGEST[/ingest]
            PROCESS[/process]
            LEDGER_R[/ledger]
            VECTOR_R[/search]
            TIC_R[/tic]
            DOMAIN_R[/domain]
            METATRON_R[/metatron]
            ZK_R[/zk]
        end

        subgraph "Handlers"
            HEALTH_H[health_handler]
            INGEST_H[ingest_handler]
            PROCESS_H[process_handler]
            LEDGER_H[ledger_handler]
            VECTOR_H[search_handler]
        end

        STATE[AppState]
    end

    ENTRY --> CONFIG
    CONFIG --> SERVER
    SERVER --> AUTH_MW
    AUTH_MW --> LOGGING
    LOGGING --> CORS
    CORS --> METRICS_MW

    METRICS_MW --> HEALTH
    METRICS_MW --> INGEST
    METRICS_MW --> PROCESS
    METRICS_MW --> LEDGER_R
    METRICS_MW --> VECTOR_R

    HEALTH --> HEALTH_H
    INGEST --> INGEST_H
    PROCESS --> PROCESS_H
    LEDGER_R --> LEDGER_H
    VECTOR_R --> VECTOR_H

    HEALTH_H --> STATE
    INGEST_H --> STATE
    PROCESS_H --> STATE
    LEDGER_H --> STATE
    VECTOR_H --> STATE
```

---

## CLI Architecture

### Command Flow

```mermaid
graph TB
    USER[User]
    CLI[mef CLI]

    subgraph "CLI Components"
        PARSER[Argument Parser - clap]
        CONFIG[Config Loader]
        DISPATCH[Command Dispatcher]

        subgraph "Commands"
            INGEST_CMD[IngestCmd]
            PROCESS_CMD[ProcessCmd]
            AUDIT_CMD[AuditCmd]
            VALIDATE_CMD[ValidateCmd]
        end

        subgraph "Execution Modes"
            API_MODE[API Mode]
            LOCAL_MODE[Local Mode]
        end
    end

    API_SERVER[API Server]
    MEF_LIB[mef-core Library]

    USER --> CLI
    CLI --> PARSER
    PARSER --> CONFIG
    CONFIG --> DISPATCH

    DISPATCH --> INGEST_CMD
    DISPATCH --> PROCESS_CMD
    DISPATCH --> AUDIT_CMD
    DISPATCH --> VALIDATE_CMD

    INGEST_CMD --> API_MODE
    INGEST_CMD --> LOCAL_MODE

    API_MODE --> API_SERVER
    LOCAL_MODE --> MEF_LIB
```

---

## Processing Pipeline

### MEF-Core Pipeline

```mermaid
graph TB
    INPUT[Input Data]

    subgraph "MEF Pipeline"
        NORMALIZE[Normalize]
        CUBE[Cube Transform]
        MANDORLA[Mandorla Filter]
        TENSOR[Resonance Tensor]
        FRACTAL[Fractal Projection]
    end

    subgraph "Spiral System"
        SNAPSHOT[Create Snapshot]
        COORDS[5D Coordinates]
        POR[Proof-of-Resonance]
    end

    subgraph "Solve-Coagula"
        XSWAP[XSwap Operations]
        SWEEP[Sweep]
        EIGEN[Eigenvalue Check]
        CONVERGE{Converged?}
    end

    TIC[TIC Crystal]
    OUTPUT[Output]

    INPUT --> NORMALIZE
    NORMALIZE --> CUBE
    CUBE --> MANDORLA
    MANDORLA --> TENSOR
    TENSOR --> FRACTAL

    FRACTAL --> SNAPSHOT
    SNAPSHOT --> COORDS
    COORDS --> POR

    POR --> XSWAP
    XSWAP --> SWEEP
    SWEEP --> EIGEN
    EIGEN --> CONVERGE
    CONVERGE -->|No| XSWAP
    CONVERGE -->|Yes| TIC

    TIC --> OUTPUT
```

---

## Ledger Architecture

### Hash-Chained Ledger

```mermaid
graph LR
    subgraph "Block 0 - Genesis"
        H0[Header]
        PH0[prev_hash: 0]
        HASH0[hash: SHA256]
        TIC0[TIC Data]
    end

    subgraph "Block 1"
        H1[Header]
        PH1[prev_hash: hash0]
        HASH1[hash: SHA256]
        TIC1[TIC Data]
    end

    subgraph "Block 2"
        H2[Header]
        PH2[prev_hash: hash1]
        HASH2[hash: SHA256]
        TIC2[TIC Data]
    end

    subgraph "Block N"
        HN[Header]
        PHN[prev_hash: hashN-1]
        HASHN[hash: SHA256]
        TICN[TIC Data]
    end

    HASH0 --> PH1
    HASH1 --> PH2
    HASH2 -.-> PHN
```

### Ledger Verification

```mermaid
sequenceDiagram
    participant Auditor
    participant Ledger
    participant Block

    Auditor->>Ledger: Request audit
    Ledger->>Ledger: Load genesis block
    loop For each block
        Ledger->>Block: Load block N
        Block->>Block: Compute hash
        Block->>Block: Verify prev_hash
        alt Hash valid
            Block-->>Ledger: ✓ Valid
        else Hash invalid
            Block-->>Ledger: ✗ Invalid at N
            Ledger-->>Auditor: Integrity failure
        end
    end
    Ledger-->>Auditor: Audit complete ✓
```

---

## Ghost Network

### Addressless Communication

```mermaid
graph TB
    subgraph "Node A"
        A_RESONANCE[Resonance State]
        A_IDENTITY[Ephemeral Identity]
        A_BROADCAST[Broadcast Engine]
        A_DISCOVERY[Discovery Engine]
    end

    subgraph "Node B"
        B_RESONANCE[Resonance State]
        B_IDENTITY[Ephemeral Identity]
        B_BROADCAST[Broadcast Engine]
        B_DISCOVERY[Discovery Engine]
    end

    subgraph "Node C"
        C_RESONANCE[Resonance State]
        C_IDENTITY[Ephemeral Identity]
        C_BROADCAST[Broadcast Engine]
        C_DISCOVERY[Discovery Engine]
    end

    subgraph "Ghost Protocol"
        ANNOUNCE[Announcements]
        DISCOVER[Discovery Messages]
        TX[Transactions]
        DECOY[Decoy Traffic]
    end

    A_BROADCAST --> ANNOUNCE
    A_DISCOVERY --> DISCOVER

    ANNOUNCE --> B_DISCOVERY
    ANNOUNCE --> C_DISCOVERY

    A_BROADCAST --> TX
    TX -.->|Route by resonance| B_BROADCAST
    TX -.->|Route by resonance| C_BROADCAST

    A_BROADCAST --> DECOY
    DECOY -.-> B_BROADCAST
    DECOY -.-> C_BROADCAST
```

### Identity Rotation

```mermaid
sequenceDiagram
    participant Node
    participant Ghost
    participant Network

    Note over Node: Initial Identity
    Node->>Ghost: Announce capabilities
    Ghost->>Network: Broadcast announcement

    loop Every N minutes
        Note over Node: Rotate Identity
        Node->>Ghost: regenerate_identity()
        Ghost->>Ghost: New ephemeral ID
        Ghost->>Network: New announcement
        Note over Ghost: Old identity forgotten
    end

    Note over Network: Cannot track node over time
```

---

## Quantum Operators

### Operator Pipeline

```mermaid
graph LR
    INPUT[Input Data]

    subgraph "Quantum Operators"
        MASK[Masking Operator]
        STEGO[Steganography Operator]
        RESONANCE[Resonance Operator]
        ZK[ZK Proof Operator]
    end

    OUTPUT[Output Data]
    PROOF[Proof/Metadata]

    INPUT --> MASK
    MASK --> STEGO
    STEGO --> RESONANCE
    RESONANCE --> ZK

    ZK --> OUTPUT
    ZK --> PROOF
```

### Masking Operation

```mermaid
sequenceDiagram
    participant User
    participant MaskOp as Masking Operator
    participant Storage

    User->>MaskOp: mask(data, seed, params)
    MaskOp->>MaskOp: Generate key from seed
    MaskOp->>MaskOp: XOR data with key
    MaskOp->>Storage: Store masked data
    Storage-->>User: masked_data

    Note over User: Later...

    User->>MaskOp: unmask(masked_data, seed)
    MaskOp->>MaskOp: Regenerate key from seed
    MaskOp->>MaskOp: XOR masked_data with key
    MaskOp-->>User: original_data
```

---

## Deployment Architectures

### Single-Node Deployment

```mermaid
graph TB
    CLIENT[Clients]

    subgraph "Single Server"
        API[API Server :8000]
        MEF[MEF-Core]
        STORAGE[Local Storage]
        LEDGER[Local Ledger]
        VECTOR[Vector DB]
    end

    CLIENT --> API
    API --> MEF
    MEF --> STORAGE
    MEF --> LEDGER
    MEF --> VECTOR
```

### Multi-Node Deployment

```mermaid
graph TB
    LB[Load Balancer]

    subgraph "API Tier"
        API1[API Server 1]
        API2[API Server 2]
        API3[API Server 3]
    end

    subgraph "Processing Tier"
        WORKER1[Worker 1]
        WORKER2[Worker 2]
    end

    subgraph "Storage Tier"
        S3[S3 Storage]
        LEDGER_CLUSTER[Ledger Cluster]
        VECTOR_CLUSTER[Vector DB Cluster]
    end

    subgraph "Monitoring"
        PROM[Prometheus]
        GRAFANA[Grafana]
    end

    LB --> API1
    LB --> API2
    LB --> API3

    API1 --> WORKER1
    API2 --> WORKER1
    API3 --> WORKER2

    WORKER1 --> S3
    WORKER1 --> LEDGER_CLUSTER
    WORKER1 --> VECTOR_CLUSTER

    WORKER2 --> S3
    WORKER2 --> LEDGER_CLUSTER
    WORKER2 --> VECTOR_CLUSTER

    API1 --> PROM
    API2 --> PROM
    API3 --> PROM
    PROM --> GRAFANA
```

### Kubernetes Deployment

```mermaid
graph TB
    INGRESS[Ingress Controller]

    subgraph "Namespace: spectralchain"
        subgraph "API Deployment"
            API_POD1[API Pod 1]
            API_POD2[API Pod 2]
            API_SVC[API Service]
        end

        subgraph "Worker Deployment"
            WORKER_POD1[Worker Pod 1]
            WORKER_POD2[Worker Pod 2]
            WORKER_SVC[Worker Service]
        end

        subgraph "Storage"
            PVC[PersistentVolumeClaim]
            S3_SECRET[S3 Credentials Secret]
            CONFIG_MAP[ConfigMap]
        end

        subgraph "Monitoring"
            METRICS_POD[Metrics Pod]
            METRICS_SVC[Metrics Service]
        end
    end

    INGRESS --> API_SVC
    API_SVC --> API_POD1
    API_SVC --> API_POD2

    API_POD1 --> WORKER_SVC
    API_POD2 --> WORKER_SVC

    WORKER_SVC --> WORKER_POD1
    WORKER_SVC --> WORKER_POD2

    WORKER_POD1 --> PVC
    WORKER_POD1 --> S3_SECRET
    WORKER_POD1 --> CONFIG_MAP

    API_POD1 --> METRICS_SVC
    WORKER_POD1 --> METRICS_SVC
    METRICS_SVC --> METRICS_POD
```

---

## Interactive Diagrams

For interactive exploration of these diagrams:
- View in GitHub (renders Mermaid automatically)
- Use [Mermaid Live Editor](https://mermaid.live/)
- Generate PNG/SVG: `make diagrams` (requires mmdc)

---

## Related Documentation

- [Quantum Resonant Architecture](./QUANTUM_RESONANT_ARCHITECTURE.md)
- [System Design Principles](./DESIGN_PRINCIPLES.md)
- [Component Interaction](./COMPONENT_INTERACTION.md)
- [API Reference](../api/README.md)

---

**Last Updated**: 2025-11-06 | **Version**: 2.0.0
