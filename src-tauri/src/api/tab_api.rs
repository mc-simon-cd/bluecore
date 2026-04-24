// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;
use crate::core::tab_manager::TabManager;

#[tauri::command]
pub async fn create_tab(url: String, tab_manager: State<'_, Arc<Mutex<TabManager>>>) -> Result<String, String> {
    let mut manager = tab_manager.lock().await;
    manager.create_tab(url)
}

#[tauri::command]
pub async fn close_tab(id: String, tab_manager: State<'_, Arc<Mutex<TabManager>>>) -> Result<bool, String> {
    let mut manager = tab_manager.lock().await;
    manager.close_tab(&id)
}
