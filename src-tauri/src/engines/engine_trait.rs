// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait BrowserEngine: Send + Sync {
    fn name(&self) -> &str;
    fn boot(&mut self) -> Result<(), String>;
    fn shutdown(&mut self) -> Result<(), String>;
    fn navigate(&self, url: &str) -> Result<(), String>;
}
