// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

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
