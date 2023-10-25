// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::windows::process::CommandExt;
use sysinfo::{ProcessExt, System, SystemExt};
use downloader::Downloader;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            on_start,
            delete_dir,
            download_standard,
            file_check,
            download_touchscreen,
            download_cyan,
            download_max98980,
            download_samus_audio,
            download_realtek,
            download_i2s,
            get_board_name,
            check_process,
            get_hwid,
            install_vc_dist,
            install,
            download_driver
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
fn create_dir() -> Option<()> {
    use std::fs;
    let _ = fs::create_dir_all("/oneclicktmp").ok()?;
     Some(())
  }
#[tauri::command]
fn delete_dir() -> Option <()> {
    use std::fs;
    let _ = fs::remove_dir_all("/oneclicktmp").ok()?;
     Some(());
     std::process::exit(0x0);
  }
#[tauri::command]
fn on_start(){
    create_dir();
}
#[tauri::command]
fn download_standard(){
    create_dir();
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("/oneclicktmp"))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl = downloader::Download::new("https://aka.ms/vs/17/release/vc_redist.x64.exe");

    let dl2 = downloader::Download::new(
        "https://github.com/coolstar/driverinstallers/raw/master/crostouchpad/crostouchpad.4.1.4-installer.exe");

    let dl3 = downloader::Download::new("https://github.com/coolstar/driverinstallers/raw/master/crosec/crosec.2.0.2-installer.exe");
    let result = downloader.download(&[dl, dl2, dl3]).unwrap();

    for r in result {
        match r {
            Err(e) => print!("Error occurred! {}", e.to_string()),
            Ok(s) => print!("Success: {}", &s),
        };
    }
}
#[tauri::command]
async fn file_check() -> String{
    use std::fs;
    let file_number = fs::read_dir("C:\\oneclicktmp").unwrap().into_iter().collect::< Vec<_>>().len();
    return file_number.to_string();
}
#[tauri::command]
fn download_touchscreen(){
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("/oneclicktmp"))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl = downloader::Download::new("https://github.com/coolstar/driverinstallers/raw/master/crostouchscreen/crostouchscreen.2.9.4-installer.exe");

    let result = downloader.download(&[dl]).unwrap();

    for r in result {
        match r {
            Err(e) => print!("Error occurred! {}", e.to_string()),
            Ok(s) => print!("Success: {}", &s),
        };
    }
}
#[tauri::command]
fn download_cyan(){
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("/oneclicktmp"))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl = downloader::Download::new("https://github.com/coolstar/driverinstallers/raw/master/max98090-r11/max98090-r11.1.0.0-installer.exe");

    let result = downloader.download(&[dl]).unwrap();

    for r in result {
        match r {
            Err(e) => print!("Error occurred! {}", e.to_string()),
            Ok(s) => print!("Success: {}", &s),
        };
    }
}
#[tauri::command]
fn download_max98980(){
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("/oneclicktmp"))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl = downloader::Download::new("https://github.com/coolstar/driverinstallers/raw/master/max98090/max98090.1.0.4-installer.exe");

    let result = downloader.download(&[dl]).unwrap();

    for r in result {
        match r {
            Err(e) => print!("Error occurred! {}", e.to_string()),
            Ok(s) => print!("Success: {}", &s),
        };
    }
}
#[tauri::command]
fn download_realtek(){
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("/oneclicktmp"))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl = downloader::Download::new("https://coolstar.org/chromebook/downloads/drivers/alc5645%20audio.exe");

    let result = downloader.download(&[dl]).unwrap();

    for r in result {
        match r {
            Err(e) => print!("Error occurred! {}", e.to_string()),
            Ok(s) => print!("Success: {}", &s),
        };
    }
}
#[tauri::command]
fn download_samus_audio(){
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("/oneclicktmp"))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl = downloader::Download::new("https://github.com/coolstar/driverinstallers/raw/master/csaudiosstcatpt/csaudiosstcatpt.1.0.1-installer.exe");

    let result = downloader.download(&[dl]).unwrap();

    for r in result {
        match r {
            Err(e) => print!("Error occurred! {}", e.to_string()),
            Ok(s) => print!("Success: {}", &s),
        };
    }
}
#[tauri::command]
fn download_i2s(){
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("/oneclicktmp"))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl = downloader::Download::new("https://coolstar.org/chromebook/downloads/drivers/BayTrailChipsetDriver-Lenovo.exe");

    let result = downloader.download(&[dl]).unwrap();

    for r in result {
        match r {
            Err(e) => print!("Error occurred! {}", e.to_string()),
            Ok(s) => print!("Success: {}", &s),
        };
    }
}

#[tauri::command]
async fn get_board_name() -> String {
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
async fn get_hwid() -> String {
    return match_result(exec("powershell", Some(vec!["Get-WmiObject", "Win32_PNPEntity", "|", "Select", "DeviceID"])));
}
#[tauri::command]
fn download_driver(value1: String){
    create_dir();
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("/oneclicktmp"))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl = downloader::Download::new(value1.as_str());

    

    let result = downloader.download(&[dl]).unwrap();

    for r in result {
        match r {
            Err(e) => print!("Error occurred! {}", e.to_string()),
            Ok(s) => print!("Success: {}", &s),
        };
    }
}

#[tauri::command]
async fn install_vc_dist(){
    let _ = exec("C:\\tmp\\vc_redist.x64.exe", Some(vec!["/quiet"]));
}
#[tauri::command]
async fn install(value1: String){
    let _ = exec(value1.as_str(), Some(vec!["/S"]));
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
