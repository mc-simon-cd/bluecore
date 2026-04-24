// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TimeRange {
    LastHour,
    LastDay,
    LastWeek,
    AllTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SitePermissions {
    pub origin: String,
    pub camera: bool,
    pub microphone: bool,
    pub notifications: bool,
    pub location: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SafetyCheckReport {
    pub compromised_passwords: u32,
    pub outdated_extensions: u32,
    pub safe_browsing_enabled: bool,
    pub update_available: bool,
}

pub struct SecurityManager {
    pub safe_browsing_enabled: bool,
    pub site_permissions: HashMap<String, SitePermissions>,
    pub incognito_mode: bool,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            safe_browsing_enabled: true,
            site_permissions: HashMap::new(),
            incognito_mode: false,
        }
    }

    pub fn run_safety_check(&self) -> SafetyCheckReport {
        // Real implementation would scan password store, extension versions, etc.
        SafetyCheckReport {
            compromised_passwords: 0,
            outdated_extensions: 0,
            safe_browsing_enabled: self.safe_browsing_enabled,
            update_available: false,
        }
    }

    pub fn clear_browser_data(&mut self, time_range: TimeRange) -> Result<String, String> {
        let label = match time_range {
            TimeRange::LastHour => "last hour",
            TimeRange::LastDay => "last 24 hours",
            TimeRange::LastWeek => "last 7 days",
            TimeRange::AllTime => "all time",
        };
        println!("Clearing browser data for: {}", label);
        // Real implementation would clear cookies, cache, and history via storage module
        Ok(format!("Browser data for '{}' cleared.", label))
    }

    pub fn get_site_permissions(&self, origin: &str) -> Option<&SitePermissions> {
        self.site_permissions.get(origin)
    }

    pub fn update_site_permissions(&mut self, permissions: SitePermissions) {
        self.site_permissions.insert(permissions.origin.clone(), permissions);
    }

    pub fn set_incognito_mode(&mut self, active: bool) {
        self.incognito_mode = active;
    }
}
