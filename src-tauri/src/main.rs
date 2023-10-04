// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod indexing;
mod init;
mod search;

use indexing::{add_folder, ls_folders, remove_folder, update};
use init::create_tables;
use search::search;

extern crate serde;
extern crate serde_json;

#[tauri::command]
fn update_indexes() -> () {
    update();
}

#[tauri::command]
fn search_indexes(query: &str) -> String {
    let results = search(&String::from(query));
    return serde_json::to_string(&results).unwrap();
}

#[tauri::command]
fn get_folders_to_index() -> String {
    let folders = ls_folders();
    return serde_json::to_string(&folders).unwrap();
}

#[tauri::command]
fn add_folders_to_index(folder: &str) -> () {
    add_folder(&String::from(folder));
}

#[tauri::command]
fn remove_folders_from_index(folder: &str) -> () {
    remove_folder(&String::from(folder));
}

fn main() {
    // Create database and tables if they don't exist
    create_tables().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            update_indexes,
            search_indexes,
            get_folders_to_index,
            add_folders_to_index,
            remove_folders_from_index,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
