// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::windows::process::CommandExt;
use sysinfo::{ProcessExt, System, SystemExt};
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_board_name,
            check_process,
            get_hwid
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_board_name() -> String {
    return match_result_vec(exec("wmic", Some(vec!["baseboard", "get", "Product"])));

}
#[tauri::command]
async fn check_process(value1: String) -> bool {
    let system = System::new_all();

    system
        .processes()
        .into_iter()
        .find(|(_, process)| {
            process.name() == value1
        })
        .is_some()
}
#[tauri::command]
fn get_hwid() -> String {
    return match_result(exec("powershell", Some(vec!["Get-WmiObject", "Win32_PNPEntity", "|", "Select", "DeviceID"])));
    }
//helper
fn exec(program: &str, args: Option<Vec<&str>>) -> Result<std::process::Output, std::io::Error> {
    let mut cmd = std::process::Command::new(program);
    #[cfg(windows)]
    cmd.creation_flags(0x08000000);
    if let Some(arg_vec) = args {
        for arg in arg_vec {
            cmd.arg(arg);
        }
    }
    return cmd.output();
}

fn match_result(result: Result<std::process::Output, std::io::Error>) -> String {
    let str = match result {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => {
            let error_string = e.to_string();
            if error_string.find("os error 2") != None {
                println!("Missing Ectools or Cbmem Binaries");
            } else {
                println!("Error `{}`.", e);
            }
            return "0".to_string();
        }
    };
    return str.trim().to_string();
}
fn match_result_vec(result: Result<std::process::Output, std::io::Error>) -> String {
    let str = match result {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>()[1]
            .clone(),
        Err(e) => {
            let error_string = e.to_string();
            if error_string.find("os error 2") != None {
                println!("Missing Ectools or Cbmem Binaries");
            } else {
                println!("Error `{}`.", e);
            }
            return "0".to_string();
        }
    };
    return str.trim().to_string();
}
