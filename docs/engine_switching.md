# Engine Switching Logic
**License:** Apache 2.0 | **Project:** Simon Project

---

## Flow Diagram
1. User requests switch or system detects trigger.
2. `EngineManager` checks `FeatureFlagController`.
3. Active engine is gracefully shutdown.
4. New engine is booted via its implementation of `EngineTrait`.

## Runtime Switches
Supported via the `engine.switch()` internal API call.
