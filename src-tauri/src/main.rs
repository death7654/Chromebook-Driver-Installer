// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::os::windows::process::CommandExt;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_board_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_board_name() -> String {
    let cmd_boardname: Result<std::process::Output, std::io::Error> =
            std::process::Command::new("wmic")
                .creation_flags(0x08000000)
                .args(["baseboard", "get", "Product"])
                .output();

        let boardnamelong: String = match cmd_boardname {
            Ok(output) => String::from_utf8_lossy(&output.stdout).split("\n").map(|x| x.to_string()).collect::<Vec<String>>()[1].clone(),
            Err(e) => {
                println!("boardnameError `{}`.", e);
                String::from("") // This match returns a blank string.
            }
        };
        let boardname = boardnamelong.trim();
        return String::from(boardname);
    }