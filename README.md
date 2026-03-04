# Syntropy - Modern State-Driven AI Agent Framework

![CI](https://github.com/KingBright/Syntropy/actions/workflows/rust.yml/badge.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.80+-orange.svg)

Syntropy is a State-of-the-Art (SOTA), production-grade AI Agent framework designed for extreme performance, memory safety, and high availability. It integrates a native, high-performance GUI (GPUI/Yororen), distributed transaction management (Saga pattern), and multimodal capabilities.

## Key Features

- **Blazing Fast UI**: Native desktop client built with GPUI and `yororen-ui`, leveraging a strict Model-View State-Driven architecture for zero-cost reactive repaints over Metal rendering on macOS.
- **Actor Model Engine**: Uses `kameo` actors and supervisor trees for resilient agent lifecycle management.
- **Distributed Saga Transactions**: Integrates `steno` to orchestrate multi-step tool calls with guaranteed compensation and rollback on failures (Chaos Engineering ready).
- **Cognitive Memory**: Built on top of SurrealDB for GraphRAG and vector embedding persistence.
- **Multimodal AI**: Direct hardware audio ingestion using `cpal` coupled with `candle-whisper` for fast local inference.
- **Secure Sandboxing**: Full Wasmtime `wasi` boundaries for untrusted execution and headless CDP browser automation.
- **Standardized IPC**: Natively implements the Model Context Protocol (MCP) and Agent Context Protocol (ACP).

## Quick Start (macOS)

### Prerequisites

1. Install Rust (varies by OS): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install standard tools: Xcode Command Line Tools.
3. *Required for UI*: Install Metal Toolchain: `sudo xcodebuild -downloadComponent MetalToolchain`

### Run the GPUI Desktop Client

```bash
cargo run --release -p syntropy-desktop
```

### Run the Headless Engine (Daemon)

```bash
cargo run --release -p syntropy-cli
```

## Workspace Structure

The framework is strictly bounded into multiple isolated crates:

- `syntropy-core`: Infrastructure, Tracing, App Config.
- `syntropy-protocols`: MCP and ACP trait boundaries.
- `syntropy-engine`: Kameo Actors and Steno Sagas execution.
- `syntropy-memory`: SurrealDB connections and Graph queries.
- `syntropy-tools`: System tools (File, Wasmtime, Browser).
- `syntropy-multimodal`: cpal Audio and candle ML models.
- `syntropy-desktop`: The GPUI application entry point.

## Contributing

See our architectural guidelines in `docs/architecture` for adding new features.
