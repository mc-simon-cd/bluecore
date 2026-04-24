# BlueCore Module System Design
**License:** Apache 2.0 | **Project:** Simon Project

---

## Module Structure
Each module lives in `/src/app/modules/` (Frontend) and `/src-tauri/src/modules/` (Backend).

## Communication
Modules MUST use the `BlueCoreAPI` to interact with browser internals.
Direct engine access is FORBIDDEN.
