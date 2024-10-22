mod download_files;

use inquire::{self, Confirm};
use std::{fs, process::exit};

//structure for device info
struct Device {
    board_name: String,
    device_name: String,
    cpu_brand: String,
    cpu_gen: String,
    touchscreen: bool,
}

const TOUCHSCREENHWID: [&str;4] = [
  "ACPI\\ATML0001",
  "ACPI\\ELAN0001",
  "ACPI\\MLFS0000",
  "ACPI\\RAYD0001",
];
const MAX989090HWID: [&str;2] = ["ACPI\\VEN_193C&DEV_9890&REV_0002", "ACPI\\193C9890"];


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

fn get_boardname() -> String {
    let cmd: Result<std::process::Output, std::io::Error> = std::process::Command::new("wmic")
        .args(vec!["baseboard", "get", "product"])
        .output();
    let str = match cmd {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .to_string()
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>()[1]
            .clone(),

        Err(e) => {
            let error = &e;
            println!("Error `{}`.", error);
            return "Error".to_string();
        }
    };
    str.trim().to_string()
}

fn main() {
    let agreement = Confirm::new("By using this application you agree to all terms and conditions in every driver you choose to install. Do you agree to these terms?").with_default(true).prompt();
    match agreement {
        Ok(true) => {
            let download_db = Confirm::new("To install your chromebook's drivers a database must be downloaded. Download Database?").with_default(true).prompt();
            match download_db {
                Ok(true) => {
                    //creates temporary download directory
                    let _ = fs::create_dir("/oneclickdriverinstalltemp");

                    //downloads database and crashes the app if an error occurs
                    let success = download_files::download("https://github.com/death7654/ChromebookDatabase/releases/latest/download/chrultrabook.db");
                    if success == "error"
                    {
                        println!("Failed to Download Database. Please restart the program.");
                        exit(0);
                    }

                    let boardname = get_boardname();
                    let hwid = get_hwid().split("\n").map(|x| x.to_string()).collect::<Vec<String>>().clone();
                    let mut touchscreenexists = false;
                    for &item in &hwid {
                        println!("{item}");
                        if TOUCHSCREENHWID.contains(&item) {
                            println!("{} exists in both arrays", item);
                            touchscreenexists = true;
                        }
                    };
                    


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
