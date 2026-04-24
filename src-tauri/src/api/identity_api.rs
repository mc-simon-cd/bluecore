// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

use std::sync::Mutex;
use tauri::State;
use crate::identity::SessionManager;
use serde::Serialize;

#[derive(Serialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub username: Option<String>,
}

#[tauri::command]
pub async fn get_session(session_manager: State<'_, Mutex<SessionManager>>) -> Result<SessionInfo, String> {
    let manager = session_manager.lock().map_err(|e| e.to_string())?;
    let session = manager.get_session();
    Ok(SessionInfo {
        session_id: session.session_id.clone(),
        username: session.user.as_ref().map(|u| u.username.clone()),
    })
}

#[tauri::command]
pub async fn login(username: String, session_manager: State<'_, Mutex<SessionManager>>) -> Result<(), String> {
    let mut manager = session_manager.lock().map_err(|e| e.to_string())?;
    manager.set_user(username);
    Ok(())
}

#[tauri::command]
pub async fn logout(session_manager: State<'_, Mutex<SessionManager>>) -> Result<(), String> {
    let mut manager = session_manager.lock().map_err(|e| e.to_string())?;
    manager.logout();
    Ok(())
}
