// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

pub trait BrowserEngine: Send + Sync {
    fn name(&self) -> &str;
    fn boot(&mut self) -> Result<(), String>;
    fn shutdown(&mut self);
    fn navigate(&self, url: &str);
}
