// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::engines::engine_trait::BrowserEngine;

pub struct ChromiumEngine;

impl BrowserEngine for ChromiumEngine {
    fn name(&self) -> &str { "Chromium Fork" }
    fn boot(&mut self) -> Result<(), String> {
        println!("Booting Chromium engine (Blink/V8)...");
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), String> {
        println!("Shutting down Chromium engine.");
        Ok(())
    }
    fn navigate(&self, url: &str) -> Result<(), String> {
        println!("Chromium navigating to: {}", url);
        Ok(())
    }
}
