use async_trait::async_trait;
use serde_json::json;
use crate::modules::{BlueCoreModule, ModuleMetadata};

pub struct PrivacyShield {
    metadata: ModuleMetadata,
}

impl PrivacyShield {
    pub fn new() -> Self {
        Self {
            metadata: ModuleMetadata {
                id: "privacy-shield".to_string(),
                name: "PrivacyShield".to_string(),
                version: "0.1.0".to_string(),
                description: "Protects your privacy by blocking trackers and ads.".to_string(),
                enabled: true,
                priority: 100,
                permissions: vec!["network_interception".to_string()],
            },
        }
    }
}

#[async_trait]
impl BlueCoreModule for PrivacyShield {
    fn metadata(&self) -> ModuleMetadata {
        self.metadata.clone()
    }

    async fn on_initialize(&self) -> Result<(), String> {
        println!("PrivacyShield module initialized");
        Ok(())
    }

    async fn on_message(&self, action: String, data: serde_json::Value) -> Result<serde_json::Value, String> {
        match action.as_str() {
            "check_url" => {
                let url = data["url"].as_str().ok_or("Missing URL")?;
                let is_blocked = url.contains("tracker") || url.contains("ads");
                Ok(json!({ "blocked": is_blocked }))
            }
            _ => Err(format!("Unknown action: {}", action)),
        }
    }
}
