// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;
use crate::core::engine_manager::EngineManager;
use crate::core::tab_manager::TabManager;

#[tauri::command]
pub async fn bc_handle_navigation(
    tab_id: String,
    url: String,
    engine_manager: State<'_, Arc<Mutex<EngineManager>>>,
    tab_manager: State<'_, Arc<Mutex<TabManager>>>,
) -> Result<(), String> {
    // 1. Sanitize the URL via TabManager if needed (TabManager::create_tab does it, but maybe navigate should too)
    // For now, let's just update the tab state and trigger the engine
    
    let mut t_manager = tab_manager.lock().await;
    let e_manager = engine_manager.lock().await;
    
    // Update tab URL in state
    t_manager.update_tab_url(&tab_id, url.clone())?;
    
    // Trigger engine navigation
    e_manager.navigate(&url)
}
