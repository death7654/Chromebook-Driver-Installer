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
const DATABASE: &str =
    "https://github.com/death7654/ChromebookDatabase/releases/latest/download/database.json";
const DATABASE_FILE_PATH: &str = "C:/oneclickdriverinstalltemp/database/database.json";

const VCREDIST: &str = "https://aka.ms/vs/17/release/vc_redist.x64.exe";

const TOUCHPAD: &str = "https://github.com/coolstar/driverinstallers/raw/master/crostouchpad/crostouchpad.4.1.6-installer.exe";
const TOUCHSCREEN: &str = "https://github.com/coolstar/driverinstallers/raw/master/crostouchscreen/crostouchscreen.2.9.5-installer.exe";
const EC: &str =
    "https://github.com/coolstar/driverinstallers/raw/master/crosec/crosec.2.0.6-installer.exe";
const CR50: &str =
    "https://github.com/coolstar/driverinstallers/raw/master/cr50/cr50.1.0.1-installer.exe";

//hwid ids
const MAX989090HWID: [&str; 2] = ["ACPI\\VEN_193C&DEV_9890&REV_0002", "ACPI\\193C9890"];
const TOUCHSCREENHWID: [&str; 4] = [
    "ACPI\\ATML0001",
    "ACPI\\MLFS0000",
    "ACPI\\RAYD0001",
    "ACPI\\ELAN0001",
];

fn get_hwid() -> Vec<String> {
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
            return vec!["Error".to_string()];
        }
    };
    let mut hwid = str
        .trim()
        .to_string()
        .split("\n")
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>()
        .clone();
    for string in &mut hwid {
        // Check if the word has at least 3 characters
        if string.len() >= 3 {
            string.truncate(string.len() - 2);
        }
    }
    return hwid;
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
async fn setup_installation() {
    //creates temporary download directory
    let _ = fs::create_dir_all("/oneclickdriverinstalltemp/drivers");
    let _ = fs::create_dir("/oneclickdriverinstalltemp/database");

    //downloads database
    let _ = download_files::download(&DATABASE, DATABASE_FILE_PATH).await;

    let _boardname: String = get_boardname();
    let hwid: Vec<String> = get_hwid();
    let mut id = 0;

    let mut touchscreenexists = false;
    let mut counter = 0;
    //println!("{:#?}", hwid.iter());

    while counter < TOUCHSCREENHWID.len() {
        if hwid.contains(&TOUCHSCREENHWID[counter].to_string()) {
            touchscreenexists = true;
            break;
        } else {
            counter += 1;
        }
    }
    println!("touchscreen? {}", touchscreenexists);

    let mut download_vector = vec![];

    let vcredist = Confirm::new("Download VC-Redist? (Required for all drivers)")
        .with_default(true)
        .prompt();
    match vcredist {
        Ok(true) => {
            //let _ = download_files::download(&VCREDIST,"C:/oneclickdriverinstalltemp/drivers/AAvcc.exe").await;
            download_vector.push(VCREDIST)
        }
        Ok(false) => {
            println!("Make sure VCREDIST is installed or in C:\\oneclickdriverinstalltemp before you install other drivers")
        }
        Err(_) => {
            println!("An Error has occured please try again")
        }
    }
    let touchpad = Confirm::new("Download touchpad drivers?")
        .with_default(true)
        .prompt();
    match touchpad {
        Ok(true) => {
            //let _ = download_files::download(&VCREDIST,"C:/oneclickdriverinstalltemp/drivers/AAvcc.exe").await;
            download_vector.push(TOUCHSCREEN)
        }
        Ok(false) => {
            println!("")
        }
        Err(_) => {
            println!("An Error has occured please try again")
        }
    }

    if touchscreenexists == true {
        let touchscreen = Confirm::new("Download touchscreen drivers?")
            .with_default(true)
            .prompt();

        match touchscreen {
            Ok(true) => {
                //let _ = download_files::download(&VCREDIST,"C:/oneclickdriverinstalltemp/drivers/AAvcc.exe").await;
                download_vector.push(TOUCHSCREEN)
            }
            Ok(false) => {
                println!("")
            }
            Err(_) => {
                println!("An Error has occured please try again")
            }
        }
    }

    //downloading section
}

fn close() {
    let cleanup = Confirm::new("Cleanup Downloaded Data?")
        .with_default(true)
        .prompt();
    match cleanup {
        Ok(true) => {
            let _ = fs::remove_dir_all("/oneclickdriverinstalltemp");
        }
        Ok(false) => {
            println!("Downloaded data is avaliable at C:/oneclickdriverinstalltemp");
            exit(0)
        }
        Err(_) => {
            println!("An Error has occured. Downloaded data is avaliable at C:/oneclickdriverinstalltemp");
            exit(0);
        }
    }
    exit(0);
}

#[tokio::main]
async fn main() {
    let agreement = Confirm::new("By using this application you agree to all terms and conditions in every driver you choose to install. Do you agree to these terms?").with_default(true).prompt();
    match agreement {
        Ok(true) => {
            let download_db = Confirm::new("To install your chromebook's drivers a database must be downloaded. Download Database?").with_default(true).prompt();
            match download_db {
                Ok(true) => {
                    let _ = setup_installation().await;
                    //when complete add win32_notif crate
                    close();
                }
                Ok(false) => {
                    println!("User denied downloading database");
                    close();
                }
                Err(_) => {
                    println!("An Error has occured please try again");
                    exit(0);
                }
            }
        }
        Ok(false) => println!("User has not accepted the agreement."),
        Err(_) => {
            println!("An Error has occured please try again")
        }
    }
}
