// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use reqwest;
use std::env;
use std::fmt::format;
use std::fs::File;
use std::str;
use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};
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
    let node_dir_srt = "node/bin/node";
    let npm_dir_srt = "node/bin";
    let output = Command::new(node_dir_srt.to_string())
        .arg("index.js")
        .arg("--version")
        .current_dir("node/lib/node_modules/npm")
        // .env("NODE_PATH", "node/lib/node_modules")
        // .env("PATH", "node/bin")
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

#[tauri::command]
async fn install_pnpm() -> Result<String, String> {
    let node_dir_srt = "node/bin/node";
    let npm_dir_srt = "node/bin";
    let current_path = env::var("PATH").unwrap_or_default();
    let output = Command::new(node_dir_srt.to_string())
        .arg("node/lib/node_modules/npm/index.js")
        .args(["install","@xhs/modular-startup"])
        // .current_dir("node/lib/node_modules/npm")
        .env("NODE_PATH", "node/node_modules")
        .env("PATH", format!("node/bin:{}",current_path))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match output {
        Ok(mut child) => {
            info!("Command started successfully. PID: {}", child.id());
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    match line {
                        Ok(line) => {
                            info!("Command output: {}", line);

                        }
                        Err(e) => {
                            error!("Error reading line from command output: {}", e);
                        }
                    }
                }
            } else {
                error!("Failed to capture stdout of the command.");
            }

            match child.wait_with_output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    if output.status.success() {
                        info!("Command completed successfully.");
                    } else {
                        error!("Command failed with stderr: {}", stderr);
                    }
                }
                Err(e) => {
                    error!("Failed to wait for command output: {}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to start command: {}", e);
        }
    }
    Ok("".to_string())

}


 fn main() {
    let timezone_offset = 8 * 3600; // 比如 UTC+5 时间区的一天的秒数

    // 创建 FixedOffset 来表示你的时区
    let timezone = FixedOffset::east(timezone_offset);
    // 配置 logger
    let config = ConfigBuilder::new()
        .set_time_offset(timezone)
        .build();
    // WriteLogger::init(
    //     LevelFilter::Info,
    //     config,
    //     File::create("/Users/fangqi/Desktop/Rust/log_file.log").unwrap(),
    // ).unwrap();

    tauri::Builder::default()
        .setup(|_app| {
            // let node_dir_srt = "node/bin/node";
            // // 启动Node.js服务器
            //  let node_output = Command::new(node_dir_srt.to_string())
            //     .arg("server.js")
            //     .current_dir("node")
            //     .spawn();
            // match node_output {
            //     Ok(child) => {
            //         info!("Server started with PID: {}", child.id());
            //     }
            //     Err(e) => {
            //         error!("Failed to start server: {}", e);
            //     }
            // };
            //
            //
            // let output = Command::new(node_dir_srt.to_string())
            //     .arg("index.js")
            //     .arg("--version")
            //     .current_dir("node/lib/node_modules/npm")
            //     .output().expect("Failed to get npm version");
            // let stdout = str::from_utf8(&output.stdout).unwrap();
            // let stderr = str::from_utf8(&output.stderr).unwrap();
            // if output.status.success() {
            //     info!("Npm version: {}", stdout);
            //     error!("Npm version: {}", stdout);
            // } else {
            //     info!("Get npm version Error: {}", stderr);
            //     error!("Get npm version Error: {}", stderr);
            // }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_app_update,
            get_npm_version,
            install_pnpm
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
