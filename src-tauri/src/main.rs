use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use tauri::Manager;
use window_vibrancy::{apply_acrylic, apply_mica};
#[derive(Debug, serde::Serialize, Deserialize, Clone)]
struct Task {
    text: String,
    completed: bool,
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "windows")]
            let _ = apply_acrylic(&window, Some((18, 18, 20, 10)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_tasks_visuals,
            add_task_visuals,
            delete_task_visuals,
            mark_done_visuals
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
fn saving(tasks: &Vec<Task>) -> std::io::Result<()> {
    let file = File::create("results.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &tasks)?;
    writer.flush()
}
fn load_tasks() -> std::io::Result<Vec<Task>> {
    let path = "results.json";
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(tasks)
}

fn add_task(tasks: &mut Vec<Task>, text: String) {
    let users_task = Task {
        text: text,
        completed: false,
    };
    tasks.push(users_task);
}

fn edit_task(tasks: &mut Vec<Task>, value: usize, input: String) {
    let new_name = Task {
        text: input,
        completed: false,
    };
    tasks[value] = new_name;
}

fn mark_done(tasks: &mut Vec<Task>, value: usize) {
    tasks[value].completed = true;
}
fn delete_task(tasks: &mut Vec<Task>, value: usize) {
    tasks.remove(value);
}
//new part the visuals but for Tauri
#[tauri::command]
fn load_tasks_visuals() -> Vec<Task> {
    load_tasks().unwrap_or_default()
}

#[tauri::command]
fn delete_task_visuals(index: usize) -> Vec<Task> {
    let mut tasks = load_tasks().unwrap_or_default();
    delete_task(&mut tasks, index);
    let _ = saving(&tasks);
    tasks
}
#[tauri::command]
fn add_task_visuals(text: String) -> Vec<Task> {
    let mut tasks = load_tasks().unwrap_or_default();
    add_task(&mut tasks, text);
    let _ = saving(&tasks);
    tasks
}
#[tauri::command]
fn mark_done_visuals(index: usize) -> Vec<Task> {
    let mut tasks = load_tasks().unwrap_or_default();
    mark_done(&mut tasks, index);
    let _ = saving(&tasks);
    tasks
}