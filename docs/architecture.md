# BlueCore Architecture
**License:** Apache 2.0 | **Project:** Simon Project

---

## Core Philosophy
"Start lightweight, scale to power engine only when needed."

## Dual-Engine System
1. **Tauri Engine**: Default rendering layer using OS-native WebView.
2. **Chromium Fork**: Inactive secondary engine for full compatibility and power.

## Internal Service Layers
- **Engine Manager**: Responsible for boot/shutdown/navigation and engine switching.
- **Identity Layer**: Manages user sessions and authentication state.
- **Tab Manager**: Orchestrates multiple browsing contexts.

## Communication Bridge
- **Tauri Invoke API**: Typescript frontend calls Rust commands via `BlueCoreAPI`.
- **State Management**: thread-safe `Mutex` wrappers for system managers.

## Modular System
- Modules (AI, Privacy, etc.) communicate through a central API.
- Deep engine abstraction via `EngineTrait` (Send + Sync).

## Engine Switch Logic
- Controlled by `config/engines.json`.
- Validated via `docs/enable_chromium.md`.
