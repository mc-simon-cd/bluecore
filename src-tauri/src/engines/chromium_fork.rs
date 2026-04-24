// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

use crate::engines::engine_trait::BrowserEngine;

pub struct ChromiumEngine;

impl BrowserEngine for ChromiumEngine {
    fn name(&self) -> &str { "Chromium Fork" }
    fn boot(&mut self) -> Result<(), String> {
        println!("Booting Chromium engine (Blink/V8)...");
        Ok(())
    }
    fn shutdown(&mut self) {
        println!("Shutting down Chromium engine.");
    }
    fn navigate(&self, url: &str) {
        println!("Chromium navigating to: {}", url);
    }
}
