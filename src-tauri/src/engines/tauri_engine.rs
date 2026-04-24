// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

use crate::engines::engine_trait::BrowserEngine;

pub struct TauriEngine;

impl BrowserEngine for TauriEngine {
    fn name(&self) -> &str { "Tauri (WebView)" }
    fn boot(&mut self) -> Result<(), String> {
        println!("Booting Tauri engine (Native WebView)...");
        Ok(())
    }
    fn shutdown(&mut self) {
        println!("Shutting down Tauri engine.");
    }
    fn navigate(&self, url: &str) {
        println!("Tauri navigating to: {}", url);
    }
}
