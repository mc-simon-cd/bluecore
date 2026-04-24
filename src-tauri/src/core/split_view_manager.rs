// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SplitPane {
    Left,
    Right,
}

impl SplitPane {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "left" => Ok(SplitPane::Left),
            "right" => Ok(SplitPane::Right),
            _ => Err(format!("Invalid pane: '{}'. Must be 'left' or 'right'.", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SplitViewState {
    pub tab_id: String,
    pub left_url: String,
    pub right_url: String,
    pub active_pane: SplitPane,
    /// 0.0 to 1.0 — percentage of width for left pane
    pub split_ratio: f32,
}

pub struct SplitViewManager {
    /// Map of tab_id to its SplitViewState
    pub split_views: HashMap<String, SplitViewState>,
}

impl SplitViewManager {
    pub fn new() -> Self {
        Self {
            split_views: HashMap::new(),
        }
    }

    pub fn enable_split_view(&mut self, tab_id: String, left_url: String, second_url: String) -> Result<SplitViewState, String> {
        let state = SplitViewState {
            tab_id: tab_id.clone(),
            left_url,
            right_url: second_url,
            active_pane: SplitPane::Left,
            split_ratio: 0.5,
        };
        self.split_views.insert(tab_id, state.clone());
        Ok(state)
    }

    pub fn set_focus(&mut self, tab_id: &str, pane: SplitPane) -> Result<(), String> {
        let view = self.split_views.get_mut(tab_id)
            .ok_or_else(|| format!("No split view for tab '{}'", tab_id))?;
        view.active_pane = pane;
        Ok(())
    }

    pub fn resize_panes(&mut self, tab_id: &str, ratio: f32) -> Result<(), String> {
        if ratio < 0.2 || ratio > 0.8 {
            return Err("Split ratio must be between 0.2 and 0.8".to_string());
        }
        let view = self.split_views.get_mut(tab_id)
            .ok_or_else(|| format!("No split view for tab '{}'", tab_id))?;
        view.split_ratio = ratio;
        Ok(())
    }

    pub fn swap_panes(&mut self, tab_id: &str) -> Result<(), String> {
        let view = self.split_views.get_mut(tab_id)
            .ok_or_else(|| format!("No split view for tab '{}'", tab_id))?;
        std::mem::swap(&mut view.left_url, &mut view.right_url);
        Ok(())
    }

    pub fn get_split_view(&self, tab_id: &str) -> Option<&SplitViewState> {
        self.split_views.get(tab_id)
    }

    pub fn disable_split_view(&mut self, tab_id: &str) -> Option<SplitViewState> {
        self.split_views.remove(tab_id)
    }
}
