#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::prelude::*;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
fn random_order(max_index: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<usize> = (0..max_index).collect();
    nums.shuffle(&mut rng);
    nums
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, random_order])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
