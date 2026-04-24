// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct ChromiumForkConfig {
    pub enabled: bool,
    pub activation_mode: String,
    pub trigger_file: String,
}


#[derive(Deserialize)]
pub struct EngineConfig {
    pub active_engine: String,
    pub chromium_fork: ChromiumForkConfig,
}

#[derive(Deserialize)]
pub struct ModuleConfigItem {
    pub id: String,
    pub enabled: bool,
    pub priority: i32,
    pub permissions: Vec<String>,
}

#[derive(Deserialize)]
pub struct ModulesConfig {
    pub modules: Vec<ModuleConfigItem>,
}

pub fn load_engine_config() -> Result<EngineConfig, String> {
    let config_path = Path::new("config/engines.json");
    let content = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read engines.json: {}", e))?;
    let config: EngineConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse engines.json: {}", e))?;
    Ok(config)
}

pub fn load_module_config() -> Result<ModulesConfig, String> {
    let config_path = Path::new("config/modules.json");
    let content = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read modules.json: {}", e))?;
    let config: ModulesConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse modules.json: {}", e))?;
    Ok(config)
}

