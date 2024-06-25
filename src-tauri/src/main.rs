// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use reqwest;
use std::env;
use std::process::Command;
use std::str;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn node_hello_express() -> Result<String, String> {
    println!("start request",);
    let resp = reqwest::get("http://localhost:3000/")
        .await
        .map_err(|e| format!("Failed to write to temp file: {}", e))?;
    println!(
        "Response: {:?}",
        resp.text()
            .await
            .map_err(|e| format!("Failed to write to temp file: {}", e))?
    );
    Ok("Hello! You've been greeted from Rust!".to_string())
}

#[tauri::command]
async fn check_app_update() -> Result<String, String> {
    println!("start check app update",);
    let resp = reqwest::get("http://localhost:3000/checkUpdate")
        .await
        .map_err(|e| format!("Failed to write to temp file: {}", e))?;
    println!(
        "Response: {:?}",
        resp.text()
            .await
            .map_err(|e| format!("Failed to write to temp file: {}", e))?
    );
    Ok("hello".to_string())
}

static NODE_DIR_STR: &str = "node/bin/node";

fn main() {
    let command_output = Command::new(NODE_DIR_STR)
        .arg("-v")
        .output()
        .expect("Failed to execute Node.js command");

    let stdout = str::from_utf8(&command_output.stdout).unwrap();
    let stderr = str::from_utf8(&command_output.stderr).unwrap();
    if command_output.status.success() {
        println!("Node.js version: {}", stdout);
        // // 更新 PATH 环境变量以包含 node 二进制目录
        let path = env::var("PATH").unwrap();
        let new_path = format!("{}:{}", NODE_DIR_STR, path);
        env::set_var("PATH", &new_path);
//         println!("Updated PATH: {}", new_path.clone());
    } else {
        println!("Error: {}", stderr);
    }

    tauri::Builder::default()
        .setup(|_| {
            // 启动Node.js服务器
            Command::new(NODE_DIR_STR)
                .arg("node/server.js")
                .spawn()
                .expect("Failed to start server");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            node_hello_express,
            check_app_update
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
