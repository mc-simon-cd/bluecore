use tauri::{State, command};
use serde_json::Value;
use crate::modules::{ModuleRegistry, ModuleMetadata};

#[command]
pub async fn get_available_modules(
    registry: State<'_, ModuleRegistry>,
) -> Result<Vec<ModuleMetadata>, String> {
    Ok(registry.get_available_modules().await)
}

#[command]
pub async fn execute_module_action(
    registry: State<'_, ModuleRegistry>,
    id: String,
    action: String,
    data: Value,
) -> Result<Value, String> {
    registry.execute_action(&id, action, data).await
}

#[command]
pub async fn toggle_module(
    _registry: State<'_, ModuleRegistry>,
    id: String,
    enabled: bool,
) -> Result<(), String> {
    println!("Toggling module {} to {}", id, enabled);
    Ok(())
}

