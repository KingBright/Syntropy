# Syntropy AI Agent - Cargo Workspace Design

To enforce strict boundary contexts and prevent monolithic coupling, Syntropy is structured as a **Cargo Workspace**. Each crate manages a specific domain, allowing granular compilation, testing, and independent deployment.

## Workspace Layout
```toml
[workspace]
resolver = "2"
members = [
    "crates/syntropy-core",
    "crates/syntropy-protocols",
    "crates/syntropy-memory",
    "crates/syntropy-tools",
    "crates/syntropy-multimodal",
    "crates/syntropy-engine",
    "bin/syntropy-cli",
    "bin/syntropy-server",
    "desktop/src-tauri"
]
```

## Crate Responsibilities

### 🔹 `crates/syntropy-core`
The backbone of the application. All other crates depend on this.
- **Responsibilities**: Shared data types, global Error enum (`thiserror`), Observability setup (`tracing-subscriber`, `opentelemetry`), Configuration loading (`figment`).
- **Key Dependencies**: `serde`, `thiserror`, `tracing`, `figment`.

### 🔹 `crates/syntropy-protocols`
Implements the external communication interfaces.
- **Responsibilities**: Model Context Protocol (MCP) server/client, Agent Client Protocol (ACP) JSON-RPC handlers, standard Prompts and Resource stream abstractions.
- **Key Dependencies**: `rmcp` (or equivalent `mcp-rust-sdk`), `jsonrpc-core`, `tokio-stream`.

### 🔹 `crates/syntropy-memory`
The Cognitive Layer & Database interactions.
- **Responsibilities**: SurrealDB connection pooling, GraphRAG query builders, Vector Embedding integrations (calling OpenAI/Local models for embeddings), and Conversation sliding-window summarizers.
- **Key Dependencies**: `surrealdb`, `async-trait`.

### 🔹 `crates/syntropy-tools`
The internal tool execution sandbox.
- **Responsibilities**: Implementations of MCP executable tools.
- **Modules**:
  - `browser`: Headless chrome automation using `headless_chrome`.
  - `sandbox`: Wasmtime + Cranelift integration for executing generated code.
  - `fs`: File system tools with strict scope limits.
- **Key Dependencies**: `headless_chrome`, `wasmtime`, `tokio`.

### 🔹 `crates/syntropy-multimodal`
Perception and sensory inputs.
- **Responsibilities**: Audio recording, VAD (Voice Activity Detection), STT (Whisper), Vision encoding (CLIP).
- **Key Dependencies**: `cpal`, `rubato`, `candle-core`, `candle-transformers`, `candle-nn`.

### 🔹 `crates/syntropy-engine`
The brain and workflow orchestrator.
- **Responsibilities**: `kameo` Actor definitions (agents), Supervision logic, `steno` Saga executions, Distributed inter-agent message routing.
- **Key Dependencies**: `kameo`, `steno`, `tokio`.

---

## Executable Targets (`bin` and UI)

### 🔸 `bin/syntropy-cli`
Command-line interface for the Agent.
- **Capabilities**: Rich terminal UI using `ratatui`, GPU image rendering (Kitty protocol). Headless daemon management.
- **Key Dependencies**: `clap`, `ratatui`, `ratatui-image`.

### 🔸 `bin/syntropy-server`
Cloud-native server binary for enterprise deployment.
- **Capabilities**: Actix-Web or Axum HTTP/SSE server for remote MCP/ACP connections, Metrics endpoint `/metrics`.
- **Key Dependencies**: `axum`, `tower-http`.

### 🔸 `desktop/src-tauri`
The GUI desktop application.
- **Capabilities**: Mac/Windows native application, system tray, permission gating via Tauri v2 plugin system.
- **Key Dependencies**: `tauri` (v2), `tauri-plugin-fs`, `tauri-plugin-dialog`.

## Code Quality & CI Rules
To maintain SOTA status, the workspace `.cargo/config.toml` will enforce:
- `#![forbid(unsafe_code)]` in all crates except where strictly necessary (e.g., C-FFI for audio/vision).
- `#![deny(clippy::unwrap_used)]` - All errors must be propagated or explicitly handled.
- Cargo Nextest for high-speed concurrent test execution.
