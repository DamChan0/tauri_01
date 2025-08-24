#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager, State};

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

// TS -> RUST

#[tauri::command]
fn get_color(state: State<Arc<Mutex<UiState>>>) -> Result<String, String> {
    let state = state.lock().unwrap().color.clone();
    Ok(state)
}

#[tauri::command]
fn set_color(
    state: State<Arc<Mutex<UiState>>>,
    app: tauri::AppHandle,
    setting_color: String,
) -> String {
    {
        let mut state = state.lock().unwrap();
        state.color = setting_color.clone();
    }
    let _ = app.emit("color_changed", &setting_color);
    setting_color
}

#[tauri::command]
fn increment(state: State<Arc<Mutex<UiState>>>, amount: Option<u32>) -> u32 {
    let mut s = state.lock().unwrap();
    s.count += amount.unwrap_or(1);
    s.count
}

#[tauri::command]
fn get_count(state: State<Arc<Mutex<UiState>>>) -> u32 {
    state.lock().unwrap().count
}

#[tauri::command]
fn set_count(state: State<Arc<Mutex<UiState>>>) -> Result<u32, ()> {
    let mut count = state.lock().unwrap().count;
    count += 1;
    Ok((count))
}

#[tauri::command]
fn get_level(state: State<'_, Arc<Mutex<UiState>>>) -> String {
    state.lock().unwrap().level.clone()
}

#[tauri::command]
fn set_level(state: State<'_, Arc<Mutex<UiState>>>, level: String) -> String {
    let mut s = state.lock().unwrap();
    s.level = level.clone();
    level
}

#[derive(Debug, Serialize)]
struct Page {
    items: Vec<String>,
    next_offset: usize,
}

#[tauri::command]
fn reset_count(state: State<'_, Arc<Mutex<UiState>>>) -> u32 {
    let mut s = state.lock().unwrap();
    s.count = 0;
    s.count
}

#[tauri::command]
fn get_items(offset: usize, limit: usize) -> Page {
    let total = 200usize;
    let end = (offset + limit).min(total);
    let item = (offset..end)
        .map(|i| format!("Item {}", i + 1))
        .collect::<Vec<_>>();
    Page {
        items: item,
        next_offset: end,
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(UiState::default())))
        .invoke_handler(tauri::generate_handler![
            get_color,
            set_color,
            get_count,
            increment,
            reset_count,
            get_level,
            set_level,
            get_items,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
