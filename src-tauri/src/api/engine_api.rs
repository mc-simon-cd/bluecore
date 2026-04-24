// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;
use tokio::sync::Mutex;
use std::fs;
use tauri::State;
use crate::core::engine_manager::EngineManager;
use crate::engines::engine_trait::BrowserEngine;
use crate::engines::tauri_engine::TauriEngine;
use crate::engines::chromium_fork::ChromiumEngine;
use crate::config::load_engine_config;

#[tauri::command]
pub async fn switch_engine(name: String, engine_manager: State<'_, Arc<Mutex<EngineManager>>>) -> Result<String, String> {
    let mut manager = engine_manager.lock().await;
    
    match name.to_lowercase().as_str() {
        "tauri" => {
            manager.switch(Box::new(TauriEngine) as Box<dyn BrowserEngine + Send + Sync>)?;
            Ok("Switched to Tauri engine".to_string())
        },
        "chromium" => {
            // Activation check
            let config = load_engine_config()?;
            let mut activated = config.chromium_fork.enabled;
            
            if !activated {
                // Check trigger file
                if let Ok(content) = fs::read_to_string(&config.chromium_fork.trigger_file) {
                    if content.contains("TOKEN: BLC-ENABLE-CHROME-V1") {
                        activated = true;
                    }
                }
            }

            if activated {
                manager.switch(Box::new(ChromiumEngine) as Box<dyn BrowserEngine + Send + Sync>)?;
                Ok("Switched to Chromium engine".to_string())
            } else {
                Err("Chromium engine is not activated. See docs/enable_chromium.md".to_string())
            }
        },
        _ => Err("Unknown engine".to_string()),
    }
}
