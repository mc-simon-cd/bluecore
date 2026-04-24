// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Navigation logic
pub struct Navigation;
impl Navigation {
    pub fn navigate(&self, url: &str) -> Result<(), String> {
        let trimmed = url.trim();
        if trimmed.is_empty() {
            return Err("Navigation URL cannot be empty".to_string());
        }
        println!("Navigating to {}", trimmed);
        Ok(())
    }
}
