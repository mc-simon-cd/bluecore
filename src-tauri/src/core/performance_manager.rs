// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PerformanceStats {
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub active_tab_count: usize,
    pub frozen_tab_count: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExperimentalFlag {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

pub struct PerformanceManager {
    pub memory_saver_enabled: bool,
    pub energy_saver_enabled: bool,
    /// Tab IDs that are excluded from memory saver (e.g., YouTube, music sites)
    pub memory_saver_exceptions: HashSet<String>,
    /// Tab IDs currently frozen by memory saver
    pub frozen_tabs: HashSet<String>,
    pub experimental_flags: Vec<ExperimentalFlag>,
}

impl PerformanceManager {
    pub fn new() -> Self {
        Self {
            memory_saver_enabled: false,
            energy_saver_enabled: false,
            memory_saver_exceptions: HashSet::new(),
            frozen_tabs: HashSet::new(),
            experimental_flags: vec![
                ExperimentalFlag {
                    id: "split-view".to_string(),
                    name: "Split View".to_string(),
                    description: "Enable dual-pane split view for tabs.".to_string(),
                    enabled: true,
                },
                ExperimentalFlag {
                    id: "reader-mode".to_string(),
                    name: "Reader Mode".to_string(),
                    description: "Immersive Reader Mode DOM sanitization.".to_string(),
                    enabled: true,
                },
                ExperimentalFlag {
                    id: "vertical-tabs".to_string(),
                    name: "Vertical Tabs".to_string(),
                    description: "Display tabs on the left side panel.".to_string(),
                    enabled: true,
                },
            ],
        }
    }

    pub fn set_memory_saver(&mut self, enabled: bool) {
        self.memory_saver_enabled = enabled;
        if !enabled {
            self.frozen_tabs.clear();
        }
    }

    pub fn set_energy_saver(&mut self, enabled: bool) {
        self.energy_saver_enabled = enabled;
    }

    pub fn freeze_tab(&mut self, tab_id: String) -> bool {
        if self.memory_saver_enabled && !self.memory_saver_exceptions.contains(&tab_id) {
            self.frozen_tabs.insert(tab_id);
            true
        } else {
            false
        }
    }

    pub fn unfreeze_tab(&mut self, tab_id: &str) {
        self.frozen_tabs.remove(tab_id);
    }

    pub fn add_memory_exception(&mut self, origin: String) {
        self.memory_saver_exceptions.insert(origin);
    }

    pub fn remove_memory_exception(&mut self, origin: &str) {
        self.memory_saver_exceptions.remove(origin);
    }

    pub fn get_stats(&self) -> PerformanceStats {
        // Real values would come from system monitoring
        PerformanceStats {
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            active_tab_count: 0,
            frozen_tab_count: self.frozen_tabs.len(),
        }
    }

    pub fn get_experimental_flags(&self) -> &Vec<ExperimentalFlag> {
        &self.experimental_flags
    }

    pub fn set_experimental_flag(&mut self, flag_id: &str, enabled: bool) -> Result<(), String> {
        if let Some(flag) = self.experimental_flags.iter_mut().find(|f| f.id == flag_id) {
            flag.enabled = enabled;
            Ok(())
        } else {
            Err(format!("Unknown flag: '{}'", flag_id))
        }
    }
}
