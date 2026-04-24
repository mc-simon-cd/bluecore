// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::engines::engine_trait::BrowserEngine;

pub struct EngineManager {
    active_engine: Box<dyn BrowserEngine + Send + Sync>,
}

impl EngineManager {
    pub fn new(default: Box<dyn BrowserEngine + Send + Sync>) -> Self {
        Self { active_engine: default }
    }

    pub fn switch(&mut self, new_engine: Box<dyn BrowserEngine + Send + Sync>) -> Result<(), String> {
        self.active_engine.shutdown()?;
        self.active_engine = new_engine;
        self.active_engine.boot()
    }

    pub fn navigate(&self, url: &str) -> Result<(), String> {
        self.active_engine.navigate(url)
    }
}
