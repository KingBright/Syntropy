# Syntropy AI Agent - Production System Architecture Spec

## 1. Executive Summary
Syntropy is a SOTA, production-grade AI Agent infrastructure built entirely in **Rust**. It shifts the AI Agent paradigm from fragile, single-threaded Python scripts to a high-availability, memory-safe, and asynchronous distributed system. 

By leveraging the Actor model (`kameo`), distributed transaction orchestration (`steno`/Saga), bidirectional communication protocols (MCP/ACP), and a native GraphRAG memory backend (`SurrealDB`), Syntropy guarantees determinism, extremely low latency in multimodal perception, and robust self-healing logic.

## 2. Production-Grade Guarantees (The "SOTA" Baseline)
As a production system, Syntropy adheres strictly to the following Engineering Non-Functional Requirements (NFRs):

### 2.1 Observability (O11y)
- **Tracing & Metrics**: Integrated `tracing` and `opentelemetry-rust`. Every Actor message, Saga transaction, and MCP RPC call inherently carries a `span_id` and `trace_id`. Traces are exported to Jaeger/OTLP.
- **Metrics**: `prometheus` endpoints for exposing CPU, memory, actor mailbox saturation, database connection pool exhaustion, and LLM token throughput.

### 2.2 Fault Tolerance & Resilience
- **Supervision Trees**: If an LLM returns malformed JSON causing a deserialization panic, the `kameo` supervisor catches the panic, isolates the failing Agent Actor, and respawns it from the last persistent snapshot without crashing the host process.
- **Saga Compensations**: All multi-step Agent operations are orchestrated via `steno`. If step 3 of a task fails, steps 1 and 2 automatically trigger their compensation (rollback) logic to prevent dirty states.

### 2.3 Security & Sandboxing
- **Execution Sandbox**: Generated code is NEVER run natively. It is compiled to WASM and executed in `wasmtime` with strict WASI limits (memory bounds, no network access, isolated virtual filesystem).
- **Zero-Trust IPC**: MCP stdio transport is constrained, and system permissions (Microphone, FS) are explicitly whitelisted via Tauri v2 capabilities.

## 3. High-Level Architecture Diagram

```mermaid
graph TD
    subgraph Client Layer
        T[Tauri v2 Desktop App]
        IDE[Code Editors - Zed/VSCode]
        API[RESTful / SSE Endpoints]
    end

    subgraph Protocol Gateway Layer
        ACPM[ACP Server - Agent Client Protocol]
        MCPM[MCP Gateway - Model Context Protocol]
        ACPM <--> |JSON-RPC| IDE
        MCPM <--> |stdio/SSE| T
    end

    subgraph Core Engine [Syntropy Actor Engine]
        SEC[Saga Execution Coordinator - steno]
        
        subgraph Agent Swarm [Kameo Actors]
            RouterAgent[Router Actor]
            CoderAgent[Coder Actor]
            BrowserAgent[Browser Actor]
        end
        
        SEC --> RouterAgent
        RouterAgent --> CoderAgent
        RouterAgent --> BrowserAgent
    end

    subgraph Multimodal Pipeline
        Audio[cpal Audio Manager] --> VAD[Silero VAD]
        VAD --> STT[candle-whisper]
        STT --> Core Engine
    end

    subgraph Tooling & Sandbox
        WASM[wasmtime Sandbox]
        Chrome[headless_chrome CDP]
        Parser[Apple Vision OCR / pdf_oxide]
    end

    subgraph Cognitive Memory
        Surreal[SurrealDB]
        Graph[Graph Relations]
        Vec[Vector Embeddings]
        Surreal --- Graph
        Surreal --- Vec
    end

    Client Layer --> Protocol Gateway Layer
    Protocol Gateway Layer --> Core Engine
    Core Engine <--> Tooling & Sandbox
    Core Engine <--> Cognitive Memory
```

## 4. Subsystem Specifications

### 4.1 Memory & Cognitive Backend: SurrealDB
- **Why SOTA**: By using SurrealDB, we eliminate the distributed monolith anti-pattern of maintaining a distinct Graph DB (e.g., Neo4j), Document DB (MongoDB), and Vector DB (Milvus).
- **Implementation**:
  - **Connection Pool**: `surrealdb` Rust crate with a `rocksdb` local backend (for desktop) or `tikv` backends (for cloud).
  - **Data Schema**: Explicit SCHEMAFULL tables for Actors, and SCHEMALESS tables for dynamic agent memory.
  - **GraphRAG**: Implementing `Vector::similarity::cosine` in SurrealQL joined with Graph edge traversals (`->mentions->entity`) directly inside the DB, preventing massive data transfer to the application layer.

### 4.2 Distributed Collaboration: Kameo + Steno
- **Actor State**: Actors implement `PersistentActor` to serialize state checkpoints. If the container drops, the agent resumes seamlessly.
- **Saga Log**: The `steno` SEC writes every state transition to SurrealDB. An API endpoint allows humans to inspect paused Sagas (e.g., waiting for User Approval) and manually resume or abort them.

### 4.3 Multimodal Perception: Native Rust Pipeline
- **Audio (cpal + candle)**: Dedicated lock-free ring buffer thread for PCM capture. No Python GIL stutters. `candle-whisper` quantized models run via Metal on Mac Studio / CUDA on Nvidia.
- **Terminal UI**: `ratatui` + `ratatui-image` utilizing the Kitty GPU rendering protocol for native chart and image rendering in the terminal shell.

## 5. Deployment Topology
- **Local Desktop Mode**: Packaged via Tauri v2. Embedded SurrealDB (RocksDB). Uses `stdio` MCP transport.
- **Cloud/Enterprise Mode**: Packaged as distroless Docker containers. Stateless Rust nodes sitting behind a Load Balancer, connecting to a remote SurrealDB cluster. Uses `HTTP/SSE` MCP transport.
