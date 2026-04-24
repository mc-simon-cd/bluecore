// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NTPSettings {
    pub background_url: Option<String>,
    pub show_shortcuts: bool,
    pub custom_shortcuts: Vec<Shortcut>,
    pub accent_color: Option<String>, // Hex color like "#4A9EFF"
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Shortcut {
    pub title: String,
    pub url: String,
    pub icon_url: Option<String>,
    pub pinned: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TabGroup {
    pub domain: String,
    pub tab_ids: Vec<String>,
    pub collapsed: bool,
}

pub struct UIConfigManager {
    pub vertical_tabs_enabled: bool,
    pub tab_sidebar_collapsed: bool,
    pub tab_groups: Vec<TabGroup>,
    pub ntp_config: NTPSettings,
    pub theme: BrowserTheme,
    pub reader_settings: ReaderSettings,
    pub active_reader_modes: HashMap<String, bool>,
}

impl UIConfigManager {
    pub fn new() -> Self {
        Self {
            vertical_tabs_enabled: false,
            tab_sidebar_collapsed: false,
            tab_groups: Vec::new(),
            ntp_config: NTPSettings {
                background_url: None,
                show_shortcuts: true,
                custom_shortcuts: Vec::new(),
                accent_color: None,
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

    pub fn set_sidebar_collapsed(&mut self, collapsed: bool) {
        self.tab_sidebar_collapsed = collapsed;
    }

    pub fn group_tabs_by_domain(&mut self, tab_domain_map: Vec<(String, String)>) {
        self.tab_groups.clear();
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();
        for (tab_id, domain) in tab_domain_map {
            groups.entry(domain).or_default().push(tab_id);
        }
        for (domain, tab_ids) in groups {
            self.tab_groups.push(TabGroup { domain, tab_ids, collapsed: false });
        }
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
