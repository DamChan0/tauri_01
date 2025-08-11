#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct UiState {
    color: String,
    count: u32,
    level: String,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            color: "#22c55e".into(),
            count: 0,
            level: "mid".into(),
        }
    }
}

struct AppState {
    counter: u32,
}

fn main() {
    idlegui_tauri_lib::run()
}
