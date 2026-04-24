// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::engines::engine_trait::BrowserEngine;

pub struct TauriEngine;

impl BrowserEngine for TauriEngine {
    fn name(&self) -> &str { "Tauri (WebView)" }
    fn boot(&mut self) -> Result<(), String> {
        println!("Booting Tauri engine (Native WebView)...");
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), String> {
        println!("Shutting down Tauri engine.");
        Ok(())
    }
    fn navigate(&self, url: &str) -> Result<(), String> {
        println!("Tauri navigating to: {}", url);
        Ok(())
    }
}
