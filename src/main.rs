use downloader::Downloader;
use inquire::{self, Confirm};
use std::{fs, process::exit};

struct Device {
    board_name: String,
    device_name: String,
    cpu_brand: String,
    cpu_gen: String,
    touchscreen: bool,
}

fn get_hwid() -> String {
    let cmd: Result<std::process::Output, std::io::Error> =
        std::process::Command::new("powershell.exe")
            .args(vec![
                "Get-WmiObject",
                "Win32_PNPEntity",
                "|",
                "Select",
                "DeviceID",
            ])
            .output();
    let str = match cmd {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => {
            let error = &e;
            println!("Error `{}`.", error);
            return "Error".to_string();
        }
    };
    str.trim().to_string()
}

fn download(link: &str) {
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("/oneclickdriverinstalltemp"))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl = downloader::Download::new(link);

    let result = downloader.download(&[dl]).unwrap();

    for r in result {
        match r {
            Err(e) => print!("Error occurred! {}", e.to_string()),
            Ok(s) => print!("Success: {}", &s),
        };
    }
}
fn get_boardname() -> String {
    let cmd: Result<std::process::Output, std::io::Error> =
        std::process::Command::new("powershell.exe")
            .args(vec![
                "Get-WmiObject",
                "Win32_PNPEntity",
                "|",
                "Select",
                "DeviceID",
            ])
            .output();
    let str = match cmd {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => {
            let error = &e;
            println!("Error `{}`.", error);
            return "Error".to_string();
        }
    };
    str.trim().to_string()
}

fn main() {
    let agreement = Confirm::new("By using this application you agree to all terms and conditions in every driver you choose to install").with_default(true).prompt();
    match agreement {
        Ok(true) => {
            let download_db = Confirm::new("To install your chromebook's drivers a database must be downloaded. Download Database?").with_default(false).prompt();
            match download_db {
                Ok(true) => {
                    let _ = fs::create_dir("/oneclickdriverinstalltemp");
                    download("https://github.com/death7654/ChromebookDatabase/releases/latest/download/chrultrabook.db");

                    let hwid: String = get_hwid();

                    let vcredist = Confirm::new("Download VC-Redist?")
                        .with_default(true)
                        .prompt();

                    let cleanup = Confirm::new("Cleanup Downloaded Data?")
                        .with_default(true)
                        .prompt();
                    match cleanup {
                        Ok(true) => {
                            let _ = fs::remove_dir_all("/oneclickdriverinstalltemp");
                        }
                        Ok(false) => {
                            println!(
                                "Downloaded data is avaliable at C:/oneclickdriverinstalltemp"
                            );
                            exit(0)
                        }
                        Err(_) => {
                            println!("An Error has occured. Downloaded data is avaliable at C:/oneclickdriverinstalltemp");
                            exit(0);
                        }
                    }
                }
                Ok(false) => {
                    println!("User denied downloading database");
                    exit(0)
                }
                Err(_) => {
                    println!("An Error has occured please try again");
                    exit(0);
                }
            }
        }
        Ok(false) => exit(0),
        Err(_) => exit(0),
    }
}
