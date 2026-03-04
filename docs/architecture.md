# Syntropy Architecture Overview

Syntropy is architected as a modular, high-performance toolkit, combining cutting-edge Rust paradigms to solve the resilience, latency, and memory safety issues found in typical Python/TypeScript AI agents.

## 1. Native GPUI Desktop Client
The `syntropy-desktop` crate bypasses Electron and Tauri web-views entirely. It uses **GPUI** (the same rendering engine as the Zed editor) to draw UI primitives directly to the GPU (Metal on macOS, Vulkan on Linux/Windows).

### State-Driven Reactive Pattern
Instead of heavy DOM diffing, Syntropy UI utilizes a centralized `Arc<Mutex<AppState>>`. Background actor threads mutate this state and call `cx.notify(entity_id)`, triggering an instant, 60fps local repaint of only the affected layout nodes. This requires explicit drop scoping (`let _ = { ... }`) to satisfy Rust's borrow checker around `gpui::Context`.

## 2. Kameo Actor Supervisor Trees
`syntropy-engine` uses `kameo` internally. Every interaction—audio stream processing, reasoning loops, model API calls—is modeled as a distinct Actor. A root Supervisor actor tracks crashes and automatically respawns child actors using Erlang-style fault tolerance.

## 3. Steno Sagas
When an AI agent decides to execute a multi-step sequence of tools (e.g., "Delete File A, Update DB B, Post Message C"), these are recorded via `steno` as a **Saga DAG (Directed Acyclic Graph)**.
If Step C fails, the SecClient automatically unwinds the graph and executes the developer-defined `compensate()` methods for Steps B and A, ensuring the system never enters an inconsistent state.

## 4. SurrealDB GraphRAG
`syntropy-memory` connects to SurrealDB. Instead of just naive Vector similarity searches, the agent maps conceptual links. "Entity A --Relates_To--> Entity B". This powers exact-match contextual inference.
