mod download_files;

use execute::generic_array::typenum::{int, False};
use execute::{shell, Execute};
use inquire::{self, Confirm};
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::process::{Command, Stdio};
use std::vec;
use std::{fs, process::exit};
use terminal_link::Link;
use zip_extract;

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

const MAXIM989090: &str =
    "https://github.com/coolstar/driverinstallers/raw/master/max98090/max98090.1.0.4-installer.exe";
const RYZEN3000AUDIO: &str = "https://github.com/coolstar/driverinstallers/raw/master/csaudioacp3x/csaudioacp3x.1.0.4-installer.exe";

const LINKI2C: &str =
    "https://github.com/coolstar/driverinstallers/raw/master/gmbusi2c/gmbusi2c.1.0-installer.exe";
const ALC5645: &str = "https://coolstar.org/chromebook/downloads/drivers/alc5645%20audio.exe";
const R11: &str = "https://github.com/coolstar/driverinstallers/raw/master/max98090-r11/max98090-r11.1.0.0-installer.exe";

const CREATIVE_AUDIO: &str =
    "https://www.dell.com/support/home/en-in/drivers/DriversDetails?driverId=4t7p8";
const AX211: &str = "https://www.intel.com/content/www/us/en/download/19351/intel-wireless-wi-fi-drivers-for-windows-10-and-windows-11.html";
const BROADWELL_RAPID_STORAGE: &str = "https://downloadcenter.intel.com/download/25910/Intel-Rapid-Storage-Technology-for-Intel-NUC?product=87570";
const COMETLAKE_RAPID_STORAGE: &str = "https://www.intel.com/content/www/us/en/download/19512/intel-rapid-storage-technology-driver-installation-software-with-intel-optane-memory-10th-and-11th-gen-platforms.html";
const XE_GRAPHICS: &str = "https://www.intel.com/content/www/us/en/download/785597/intel-arc-iris-xe-graphics-windows.html?wapkw=xe-graphics";
const JASPERLAKE_GRAPHICS: &str = "https://www.intel.com/content/www/us/en/download/776137/intel-7th-10th-gen-processor-graphics-windows.html?wapkw=intel%20hd";
const AMD_CHIPSET: &str = "https://www.amd.com/en/support/downloads/drivers.html/chipsets/laptop-chipsets/amd-ryzen-and-athlon-mobile-chipset.html";
const AMD_GRAPHICS: &str = "https://www.amd.com/en/support/download/drivers.html";

const PURCHASE: &str = "https://coolstar.org/chromebook/driverlicense/login.html";
//hwid ids
const MAX989090HWID: [&str; 2] = ["ACPI\\VEN_193C&DEV_9890&REV_0002", "ACPI\\193C9890"];
const TOUCHSCREENHWID: [&str; 4] = [
    "ACPI\\ATML0001",
    "ACPI\\MLFS0000",
    "ACPI\\RAYD0001",
    "ACPI\\ELAN0001",
];

const AUTO_INSTALL_INTEL_CHIPSET_PS1: &str =
    "https://raw.githubusercontent.com/coolstar/driverinstallers/master/autoinstall-intel.zip";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Chromebook {
    cpu_codename: String,
    avaliable_drivers: String,
    cpu_brand: String,
    device_name: String,
    cpu_generation: String,
    board_name: String,
    touchscreen: bool,
}

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

fn remove_quotes(input: String) -> String {
    input[1..(input.len() - 1)].to_string()
}
async fn download_relay(list: Vec<&str>) {
    let mut counter = 0;
    for i in list {
        if i != AUTO_INSTALL_INTEL_CHIPSET_PS1 {
            let path = "/oneclickdriverinstalltemp/drivers/".to_string()
                + (&counter.to_string())
                + &".exe";
            let _ = download_files::download(i, &path).await;
        } else {
            let _ = download_files::download(i, "/oneclickdriverinstalltemp/zip/intel.zip").await;
        }
        counter += 1;
    }
}

async fn setup_installation() {
    //creates temporary download directory
    let _ = fs::create_dir_all("/oneclickdriverinstalltemp/drivers");
    let _ = fs::create_dir("/oneclickdriverinstalltemp/database");

    //downloads database
    let _ = download_files::download(&DATABASE, DATABASE_FILE_PATH).await;

    //converts the .json file into a string
    let boardname: String = get_boardname();
    let mut file = File::open("/oneclickdriverinstalltemp/database/database.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //converts the string into json objects
    let v: Value = serde_json::from_str(&mut contents).unwrap();

    //empty object to store future detected values
    let mut chromebooks = Chromebook {
        cpu_codename: String::new(),
        avaliable_drivers: String::new(),
        cpu_brand: String::new(),
        device_name: String::new(),
        cpu_generation: String::new(),
        board_name: String::new(),
        touchscreen: false,
    };
    //converts json objects into an array so it can be iterated upon
    let objects = v.as_array().unwrap().clone();
    let mut counter = 0;

    //iterating through an array
    for _ in &objects {
        if objects[counter]["board_name"] == boardname {
            chromebooks.cpu_codename = remove_quotes(objects[counter]["cpu_codename"].to_string());
            chromebooks.avaliable_drivers =
                remove_quotes(objects[counter]["avaliable_drivers"].to_string());
            chromebooks.cpu_brand = remove_quotes(objects[counter]["cpu_brand"].to_string());
            chromebooks.device_name = remove_quotes(objects[counter]["device_name"].to_string());
            chromebooks.board_name = remove_quotes(objects[counter]["board_name"].to_string());
            chromebooks.cpu_generation =
                remove_quotes(objects[counter]["cpu_generation"].to_string());
        }
        counter += 1
    }

    let hwid: Vec<String> = get_hwid(); //physical device hardware id (elan0001)
    counter = 0;

    while counter < TOUCHSCREENHWID.len() {
        if hwid.contains(&TOUCHSCREENHWID[counter].to_string()) {
            chromebooks.touchscreen = true;
            break;
        } else {
            counter += 1;
        }
    }

    if chromebooks.device_name.len() > 1 {
        println!(
            "\n\nYour device has been detected as \n\n {:#?}",
            chromebooks
        );
    } else {
        let option = Confirm::new(
            "Your device has not been properly detected or is not in the database. Continue?",
        )
        .with_default(true)
        .prompt();
        match option {
            Ok(true) => {}
            Ok(false) => exit(0),
            Err(_) => exit(0),
        }
    }
    if chromebooks.board_name == "Stout" {
        println!("Your Chromebook Has No Avaliable Drivers. The Program will now exit");
        exit(0);
    }

    let mut download_vector = vec![];

    let vcredist = Confirm::new("Download VC-Redist? (Required for all drivers)")
        .with_default(true)
        .prompt();
    match vcredist {
        Ok(true) => download_vector.push(VCREDIST),
        Ok(false) => {
            println!("Make sure VCREDIST is installed or in C:\\oneclickdriverinstalltemp before you install other drivers")
        }
        Err(_) => {
            println!("An Error has occured please try again");
            exit(0)
        }
    }
    let touchpad = Confirm::new("Download touchpad drivers?")
        .with_default(true)
        .prompt();
    match touchpad {
        Ok(true) => download_vector.push(TOUCHPAD),
        Ok(false) => {
            println!("")
        }
        Err(_) => {
            println!("An Error has occured please try again");
            exit(0)
        }
    }
    let ec = Confirm::new("Download ec driver?")
        .with_default(true)
        .prompt();
    match ec {
        Ok(true) => download_vector.push(EC),
        Ok(false) => {
            println!("")
        }
        Err(_) => {
            println!("An Error has occured please try again");
            exit(0)
        }
    }

    if chromebooks.touchscreen == true {
        let touchscreen = Confirm::new("Download touchscreen drivers?")
            .with_default(true)
            .prompt();

        match touchscreen {
            Ok(true) => download_vector.push(TOUCHSCREEN),
            Ok(false) => {}
            Err(_) => {
                println!("An Error has occured please try again");
                exit(0)
            }
        }
    }
    let mut max989090 = false;
    counter = 0;
    while counter < MAX989090HWID.len() {
        if hwid.contains(&MAX989090HWID[counter].to_string()) {
            max989090 = true;
            break;
        } else {
            counter += 1;
        }
    }

    if max989090 == true {
        let max = Confirm::new("Download Maxim989090 audio drivers?")
            .with_default(true)
            .prompt();

        match max {
            Ok(true) => download_vector.push(MAXIM989090),
            Ok(false) => {}
            Err(_) => {
                println!("An Error has occured please try again");
                exit(0)
            }
        }
    }

    if chromebooks.board_name == "Link" {
        let i2c = Confirm::new("Download the i2c driver?")
            .with_default(true)
            .prompt();

        match i2c {
            Ok(true) => download_vector.push(LINKI2C),
            Ok(false) => {}
            Err(_) => {
                println!("An Error has occured please try again");
                exit(0)
            }
        }
        let creative_audio = Link::new("Creative Audio Driver download link", CREATIVE_AUDIO);
        println!("Due to Legal Constraints, Please download the Creative Audio Driver and move it to C:/oneclickdriverinstalltemp \n\n{}", creative_audio);
    }

    if chromebooks.avaliable_drivers.contains("alc5645-audio") {
        let alc5645 = Confirm::new("Download the ALC5465 audio driver?")
            .with_default(true)
            .prompt();

        match alc5645 {
            Ok(true) => download_vector.push(ALC5645),
            Ok(false) => {}
            Err(_) => {
                println!("An Error has occured please try again");
                exit(0)
            }
        }
    }

    if chromebooks.board_name == "Cyan" {
        let r11 = Confirm::new("Download the r11 audio driver?")
            .with_default(true)
            .prompt();

        match r11 {
            Ok(true) => download_vector.push(R11),
            Ok(false) => {}
            Err(_) => {
                println!("An Error has occured please try again");
                exit(0)
            }
        }
    }

    if chromebooks
        .avaliable_drivers
        .contains("rapid-storage-broadwell")
    {
        let broadwell_rapid_storage = Link::new(
            "Rapid Storage driver download link",
            BROADWELL_RAPID_STORAGE,
        );
        println!("Due to Legal Constraints, Please download the Rapid Storage driver and move it to C:/oneclickdriverinstalltemp. Although not necessary, Intel's version is specialized and provides better battery life. \n\n{}", broadwell_rapid_storage);
    }
    if chromebooks.avaliable_drivers.contains("AX211-Wifi") {
        let ax211_wifi = Link::new("ax211 wifi download link", AX211);
        println!("Due to Legal Constraints, Please download the Intel Wifi driver and move it to C:/oneclickdriverinstalltemp \n\n{}", ax211_wifi);
    }
    if chromebooks.avaliable_drivers.contains("XE-Graphics") {
        let xe = Link::new("graphics driver download link", XE_GRAPHICS);
        println!("Due to Legal Constraints, Please download the graphics driver and move it to C:/oneclickdriverinstalltemp \n\n{}", xe);
    }
    if chromebooks.avaliable_drivers.contains("CezanneChipset")
        || chromebooks.avaliable_drivers.contains("MendocinoChipset")
        || chromebooks
            .avaliable_drivers
            .contains("picasso/dalichipset")
    {
        let amd = Link::new("AMD Chipset Drivers download link", AMD_CHIPSET);
        println!("Due to Legal Constraints, Please download the AMD Chipset Drivers and move it to C:/oneclickdriverinstalltemp \n\n{}", amd);
    }
    if chromebooks.avaliable_drivers.contains("Radeon-Graphics")
        || chromebooks.avaliable_drivers.contains("Radeon-GPU")
        || chromebooks.avaliable_drivers.contains("vegagpu")
    {
        let graphics = Link::new("Amd Graphics Driver download link", AMD_GRAPHICS);
        println!("Due to Legal Constraints, Please download the AMD Graphics Drivers and move it to C:/oneclickdriverinstalltemp \n\n{}", graphics);
    }
    if chromebooks.avaliable_drivers.contains("hd-graphics") {
        let graphics = Link::new("Graphics Driver download link", JASPERLAKE_GRAPHICS);
        println!("Due to Legal Constraints, Please download the Graphics Drivers and move it to C:/oneclickdriverinstalltemp \n\n{}", graphics);
    }
    if chromebooks.avaliable_drivers.contains("CR50") {
        let cr50 = Confirm::new("Download the CR50 driver?")
            .with_default(true)
            .prompt();

        match cr50 {
            Ok(true) => download_vector.push(CR50),
            Ok(false) => {}
            Err(_) => {
                println!("An Error has occured please try again");
                exit(0)
            }
        }
    }
    if chromebooks.avaliable_drivers.contains("ryzen3000-audio") {
        let ryzen3000 = Confirm::new("Download the Ryzen 3000 audio driver?")
            .with_default(true)
            .prompt();

        match ryzen3000 {
            Ok(true) => download_vector.push(RYZEN3000AUDIO),
            Ok(false) => {}
            Err(_) => {
                println!("An Error has occured please try again");
                exit(0)
            }
        }
    }
    if chromebooks
        .avaliable_drivers
        .contains("rapid-storage-cometlake")
    {
        let graphics = Link::new("Rapid Storage download link", COMETLAKE_RAPID_STORAGE);
        println!("Due to Legal Constraints, Please download the Rapid Storage and move it to C:/oneclickdriverinstalltemp \n\n{}", graphics);
    }
    if chromebooks.avaliable_drivers.contains("AlderLakeChipset")
        || chromebooks.avaliable_drivers.contains("TigerLakeChipset")
        || chromebooks.avaliable_drivers.contains("jasperlakechipset")
    {
        let chipset = Confirm::new("Download the Intel chipset driver?")
            .with_default(true)
            .prompt();

        match chipset {
            Ok(true) => {
                download_vector.push(AUTO_INSTALL_INTEL_CHIPSET_PS1);
                let _ = fs::create_dir_all("/oneclickdriverinstalltemp/zip");
            }
            Ok(false) => {}
            Err(_) => {
                println!("An Error has occured please try again");
                exit(0)
            }
        }
    }
    if chromebooks.avaliable_drivers.contains("cAVS")
        || chromebooks.avaliable_drivers.contains("cSOF")
        || chromebooks.avaliable_drivers.contains("sof")
        || chromebooks.avaliable_drivers.contains("Thunderbolt-4")
        || chromebooks.avaliable_drivers.contains("sof-amd")
    {
        let driver_purchase = Link::new("Store link", PURCHASE);
        println!(
            "Your chromebook has audio or thunderbolt drivers avaliable to be purchased. \n\n{}",
            driver_purchase
        );
    }

    //downloading section
    download_relay(download_vector).await;
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
