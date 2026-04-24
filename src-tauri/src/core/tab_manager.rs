// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

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

    pub fn create_tab(&mut self, url: String) -> String {
        let id = Uuid::new_v4().to_string();
        let tab = Tab { id: id.clone(), url };
        self.tabs.insert(id.clone(), tab);
        id
    }

    pub fn close_tab(&mut self, id: &str) -> bool {
        self.tabs.remove(id).is_some()
    }

    pub fn list_tabs(&self) -> Vec<&Tab> {
        self.tabs.values().collect()
    }
}
