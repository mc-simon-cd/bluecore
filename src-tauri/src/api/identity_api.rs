// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;
use crate::identity::SessionManager;
use serde::Serialize;

#[derive(Serialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub username: Option<String>,
}

#[tauri::command]
pub async fn get_session(session_manager: State<'_, Arc<Mutex<SessionManager>>>) -> Result<SessionInfo, String> {
    let manager = session_manager.lock().await;
    let session = manager.get_session();
    Ok(SessionInfo {
        session_id: session.session_id.clone(),
        username: session.user.as_ref().map(|u| u.username.clone()),
    })
}

#[tauri::command]
pub async fn login(username: String, session_manager: State<'_, Arc<Mutex<SessionManager>>>) -> Result<(), String> {
    let mut manager = session_manager.lock().await;
    manager.set_user(username);
    Ok(())
}

#[tauri::command]
pub async fn logout(session_manager: State<'_, Arc<Mutex<SessionManager>>>) -> Result<(), String> {
    let mut manager = session_manager.lock().await;
    manager.logout();
    Ok(())
}
