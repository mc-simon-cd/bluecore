// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use uuid::Uuid;

pub struct Tab {
    pub id: String,
    pub url: String,
}

pub struct TabManager {
    tabs: HashMap<String, Tab>,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: HashMap::new(),
        }
    }

    pub fn create_tab(&mut self, url: String) -> Result<String, String> {
        // Basic URL sanitization & validation
        let sanitized_url = self.sanitize_url(url)?;
        
        let id = Uuid::new_v4().to_string();
        let tab = Tab { id: id.clone(), url: sanitized_url };
        self.tabs.insert(id.clone(), tab);
        Ok(id)
    }

    fn sanitize_url(&self, url: String) -> Result<String, String> {
        let trimmed = url.trim();
        if trimmed.is_empty() {
            return Err("URL cannot be empty".to_string());
        }

        // Basic protection against local file access if not explicitly allowed
        if trimmed.starts_with("file://") {
            return Err("Local file access via URL is restricted".to_string());
        }

        // Ensure it has a protocol or prepend https://
        if !trimmed.contains("://") {
            Ok(format!("https://{}", trimmed))
        } else {
            Ok(trimmed.to_string())
        }
    }

    pub fn close_tab(&mut self, id: &str) -> Result<bool, String> {
        if self.tabs.remove(id).is_some() {
            Ok(true)
        } else {
            Err(format!("Tab with ID {} not found", id))
        }
    }

    pub fn update_tab_url(&mut self, id: &str, url: String) -> Result<(), String> {
        let sanitized = self.sanitize_url(url)?;
        if let Some(tab) = self.tabs.get_mut(id) {
            tab.url = sanitized;
            Ok(())
        } else {
            Err(format!("Tab with ID {} not found", id))
        }
    }

    pub fn list_tabs(&self) -> Vec<&Tab> {
        self.tabs.values().collect()
    }
}
