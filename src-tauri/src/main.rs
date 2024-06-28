// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use reqwest;
use std::env;
use std::fs::File;
use std::str;
use std::io::Read;
use std::process::{Command};
use serde_json::{json, Value};
use simplelog::*;
use log::{error, info}; // 导入log宏
use chrono::offset::FixedOffset;

#[tauri::command]
async fn check_app_update() -> Result<Value, Value> {
    println!("start check app update",);
    let response = reqwest::get("https://localhost:3000/checkUpdate")
        .await
        .map_err(|e| format!("Failed to write to temp file: {}", e))?;
        let json: Value = response.json().await.map_err(|e| format!("Failed to write to temp file: {}", e))?;
    if let Some(needUpdate) = json.get("needUpdate") {
        println!("needUpdate: {}", needUpdate);
    } else {
        println!("Key 'message' not found in the JSON response.");
    }
    Ok(json)
}

#[tauri::command]
async fn get_npm_version() -> Result<String, String> {
    let node_dir_srt = "/Applications/tauri-updater.app/Contents/Resources/node/bin/node";
    let npm_dir_srt = "/Applications/tauri-updater.app/Contents/Resources/node/bin";
    let output = Command::new(node_dir_srt.to_string())
        .arg("index.js")
        .arg("--version")
        .current_dir("/Applications/tauri-updater.app/Contents/Resources/node/lib/node_modules/npm")
        // .env("NODE_PATH", "/Applications/tauri-updater.app/Contents/Resources/node/lib/node_modules")
        // .env("PATH", "/Applications/tauri-updater.app/Contents/Resources/node/bin")
        .output().expect("Failed to get npm version");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let stderr = str::from_utf8(&output.stderr).unwrap();
    if output.status.success() {
        info!("Npm version: {}", stdout);
        error!("Npm version: {}", stdout);
          Ok(format!("current version: {}",stdout))
    } else {
        info!("Get npm version Error: {}", stderr);
        error!("Get npm version Error: {}", stderr);
        Err(format!("get version error: {}",stderr))
    }

}


 fn main() {
    let timezone_offset = 8 * 3600; // 比如 UTC+5 时间区的一天的秒数

    // 创建 FixedOffset 来表示你的时区
    let timezone = FixedOffset::east(timezone_offset);
    // 配置 logger
    let config = ConfigBuilder::new()
        .set_time_offset(timezone)
        .build();
    WriteLogger::init(
        LevelFilter::Info,
        config,
        File::create("/Users/fangqi/Desktop/Rust/log_file.log").unwrap(),
    ).unwrap();

    tauri::Builder::default()
        .setup(|_app| {
            let node_dir_srt = "/Applications/tauri-updater.app/Contents/Resources/node/bin/node";
            // 启动Node.js服务器
             let node_output = Command::new(node_dir_srt.to_string())
                .arg("server.js")
                .current_dir("/Applications/tauri-updater.app/Contents/Resources/node")
                .spawn();
            match node_output {
                Ok(child) => {
                    info!("Server started with PID: {}", child.id());
                }
                Err(e) => {
                    error!("Failed to start server: {}", e);
                }
            };


            let output = Command::new(node_dir_srt.to_string())
                .arg("index.js")
                .arg("--version")
                .current_dir("/Applications/tauri-updater.app/Contents/Resources/node/lib/node_modules/npm")
                .output().expect("Failed to get npm version");
            let stdout = str::from_utf8(&output.stdout).unwrap();
            let stderr = str::from_utf8(&output.stderr).unwrap();
            if output.status.success() {
                info!("Npm version: {}", stdout);
                error!("Npm version: {}", stdout);
            } else {
                info!("Get npm version Error: {}", stderr);
                error!("Get npm version Error: {}", stderr);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_app_update,
            get_npm_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
