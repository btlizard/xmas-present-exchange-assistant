#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::prelude::*;
use rusqlite::{params, Connection, Result, NO_PARAMS};
use tauri::utils::config::UpdaterEndpoint;
use std::env;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

#[derive(Debug)]
struct Player {
    id: i32,
    name: String,
}

#[tauri::command]
fn random_order(max_index: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<usize> = (0..max_index).collect();
    nums.shuffle(&mut rng);
    nums
}

#[tauri::command]
fn get_players() -> Vec<String> {
    let conn = Connection::open("xpx.db").unwrap();
    conn.execute(
        "create table if not exists players (
            id integer primary key,
            name text not null 
        )",
        [],
    ).unwrap();
    let mut stmt =  conn.prepare( "SELECT id, name FROM players").unwrap();
    let player_iter = stmt .query_map([], |row| {
            Ok(Player {
                id: row.get(0)?,
                name: row.get(1)?,
            })
    }).unwrap();
    let mut players = Vec::new();
    for p in player_iter {
        if let Ok(player) = p {
            players.push(player.name);
        }
    }
    players
}

#[tauri::command]
fn add_player(name: String) {
    let conn = Connection::open("xpx.db").unwrap();
    conn.execute(
        "INSERT INTO players (name) VALUES (:name)", 
        &[(":name", &name)],
    ).unwrap();
    println!("!!!! add player done!")
}

#[tauri::command]
fn remove_player(name: String) {
    println!(" delete name : {}", &name);
    let conn = Connection::open("xpx.db").unwrap();
    match conn.execute(
        "DELETE FROM players WHERE name=(:name)", 
        &[(":name", &name)],
    ) {
        Ok(deleted) => println!("{} rows were deleted", deleted),
        Err(err) => println!("remove failed: {}", err),
    }
}

#[tauri::command]
fn get_current_dir() -> String {
    let path = env::current_dir().unwrap();
    path.as_os_str().to_str().unwrap().to_string()
}
fn main() {
    // let about = CustomMenuItem::new("about".to_string(), "About");
    // let menu = Menu::new()
    //     .add_item(CustomMenuItem::new("about", "About"));
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            random_order, 
            get_players,
            add_player,
            remove_player,
            get_current_dir
        ]) .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
