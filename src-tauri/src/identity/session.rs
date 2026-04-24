// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
}

pub struct Session {
    pub session_id: String,
    pub user: Option<User>,
}

pub struct SessionManager {
    current_session: Session,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            current_session: Session {
                session_id: Uuid::new_v4().to_string(),
                user: None,
            },
        }
    }

    pub fn get_session(&self) -> &Session {
        &self.current_session
    }

    pub fn set_user(&mut self, username: String) {
        self.current_session.user = Some(User {
            id: Uuid::new_v4().to_string(),
            username,
        });
    }

    pub fn logout(&mut self) {
        self.current_session.user = None;
    }
}
