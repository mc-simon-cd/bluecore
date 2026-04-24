use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

pub mod privacy_shield;

#[derive(Clone, Serialize, Deserialize)]
pub struct ModuleMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub enabled: bool,
    pub priority: i32,
    pub permissions: Vec<String>,
}

#[async_trait]
pub trait BlueCoreModule: Send + Sync {
    fn metadata(&self) -> ModuleMetadata;
    async fn on_initialize(&self) -> Result<(), String>;
    async fn on_message(&self, action: String, data: serde_json::Value) -> Result<serde_json::Value, String>;
}

#[derive(Clone)]
pub struct ModuleRegistry {
    modules: Arc<RwLock<HashMap<String, Box<dyn BlueCoreModule>>>>,
}


impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, module: Box<dyn BlueCoreModule>) {
        let mut modules_guard = self.modules.write().await;
        let modules: &mut HashMap<String, Box<dyn BlueCoreModule>> = &mut *modules_guard;
        modules.insert(module.metadata().id.clone(), module);
    }

    pub async fn get_available_modules(&self) -> Vec<ModuleMetadata> {
        let modules_guard = self.modules.read().await;
        let modules: &HashMap<String, Box<dyn BlueCoreModule>> = &*modules_guard;
        modules.values().map(|m: &Box<dyn BlueCoreModule>| m.metadata()).collect()
    }

    pub async fn execute_action(&self, id: &str, action: String, data: serde_json::Value) -> Result<serde_json::Value, String> {
        let modules_guard = self.modules.read().await;
        let modules: &HashMap<String, Box<dyn BlueCoreModule>> = &*modules_guard;
        if let Some(module) = modules.get(id) {
            module.on_message(action, data).await
        } else {
            Err(format!("Module {} not found", id))
        }
    }


}
