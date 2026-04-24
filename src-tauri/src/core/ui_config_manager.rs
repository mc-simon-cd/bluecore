// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NTPSettings {
    pub background_url: Option<String>,
    pub show_shortcuts: bool,
    pub custom_shortcuts: Vec<Shortcut>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Shortcut {
    pub title: String,
    pub url: String,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BrowserTheme {
    Light,
    Dark,
    System,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReaderSettings {
    pub font_size: u8,
    pub theme: String, // "sepia", "night", "default"
    pub font_family: String,
}

pub struct UIConfigManager {
    pub vertical_tabs_enabled: bool,
    pub ntp_config: NTPSettings,
    pub theme: BrowserTheme,
    pub reader_settings: ReaderSettings,
    pub active_reader_modes: HashMap<String, bool>, // tab_id -> is_active
}

impl UIConfigManager {
    pub fn new() -> Self {
        Self {
            vertical_tabs_enabled: false,
            ntp_config: NTPSettings {
                background_url: None,
                show_shortcuts: true,
                custom_shortcuts: Vec::new(),
            },
            theme: BrowserTheme::System,
            reader_settings: ReaderSettings {
                font_size: 16,
                theme: "default".to_string(),
                font_family: "Inter".to_string(),
            },
            active_reader_modes: HashMap::new(),
        }
    }

    pub fn set_vertical_tabs(&mut self, enabled: bool) {
        self.vertical_tabs_enabled = enabled;
    }

    pub fn update_ntp_setting(&mut self, settings: NTPSettings) {
        self.ntp_config = settings;
    }

    pub fn set_theme(&mut self, theme: BrowserTheme) {
        self.theme = theme;
    }

    pub async fn set_reader_mode(&mut self, tab_id: String, active: bool) -> Result<bool, String> {
        self.active_reader_modes.insert(tab_id, active);
        Ok(active)
    }
}
