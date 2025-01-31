mod download_files;
mod helper;

use inquire::{self, Confirm};
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::vec;
use std::{fs, process::exit};
use terminal_link::Link;

use std::process::ExitStatus;
use std::thread::sleep;
use std::time::Duration;


use zip_extensions::*;

const DATABASE: &str =
    "https://github.com/death7654/ChromebookDatabase/releases/latest/download/database.json";

const LINKS: &str = "https://github.com/death7654/Driver-Installer-Links/releases/latest/download/links.json";
const DATABASE_FILE_PATH: &str = "C:/oneclickdriverinstalltemp/database/database.json";

const LINKS_FILE_PATH: &str = "C:/oneclickdriverinstalltemp/database/links.json";

const MAX989090HWID: [&str; 2] = ["ACPI\\VEN_193C&DEV_9890&REV_0002", "ACPI\\193C9890"];
const TOUCHSCREENHWID: [&str; 4] = [
    "ACPI\\ATML0001",
    "ACPI\\MLFS0000",
    "ACPI\\RAYD0001",
    "ACPI\\ELAN0001",
];

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
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Links {
    vcredist: String,
    touchpad: String,
    touchscreen: String,
    ec: String,
    wilco_ec: String,
    cr50: String,
    maxim989090: String,
    ryzen3000audio: String,
    i2c_link: String,
    alc5645: String,
    cyan_audio: String,
    creative_audio: String,
    r11_audio: String,
    drallion_audio: String,
    ax211_wifi: String,
    broadwell_rapid_storage: String,
    cometlake_rapid_storage:String,
    xe_graphics: String,
    jasperlake_chipset: String,
    amd_chipset:String,
    amd_graphics: String,
    intel_chipset: String,
    chrultrabook_tools: String,
    purchase_driver_portal: String,
}

async fn download_relay(list: Vec<String>) {
    let mut counter = 0;
    for i in list {
        if i != "https://raw.githubusercontent.com/coolstar/driverinstallers/master/autoinstall-intel.zip" {
            let path = "/oneclickdriverinstalltemp/drivers/".to_string()
                + (&counter.to_string())
                + &".exe";
            let _ = download_files::download(&i, &path).await;
        } else {
            let _ = download_files::download(&i, "/oneclickdriverinstalltemp/zip/intel.zip").await;
        }
        counter += 1;
    }
}

async fn setup_installation() -> Vec<String> {
    //creates temporary download directory
    let _ = fs::create_dir_all("/oneclickdriverinstalltemp/drivers");
    let _ = fs::create_dir("/oneclickdriverinstalltemp/database");

    //downloads database
    let _ = download_files::download(&DATABASE, DATABASE_FILE_PATH).await;
    let _ = download_files::download(&LINKS, LINKS_FILE_PATH).await;

    //gets boardname
    let boardname: String = helper::get_boardname();

    //converts the .json file into a string
    let mut file = File::open("/oneclickdriverinstalltemp/database/database.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //converts the string into json objects
    let mut v: Value = serde_json::from_str(&mut contents).unwrap();

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
    let mut objects = v.as_array().unwrap().clone();
    let mut counter = 0;

    //iterating through an array
    for _ in &objects {
        if objects[counter]["board_name"] == boardname {
            chromebooks.cpu_codename =
                helper::remove_quotes(objects[counter]["cpu_codename"].to_string());
            chromebooks.avaliable_drivers =
                helper::remove_quotes(objects[counter]["avaliable_drivers"].to_string());
            chromebooks.cpu_brand =
                helper::remove_quotes(objects[counter]["cpu_brand"].to_string());
            chromebooks.device_name =
                helper::remove_quotes(objects[counter]["device_name"].to_string());
            chromebooks.board_name =
                helper::remove_quotes(objects[counter]["board_name"].to_string());
            chromebooks.cpu_generation =
                helper::remove_quotes(objects[counter]["cpu_generation"].to_string());
        }
        counter += 1
    }

    file = File::open("/oneclickdriverinstalltemp/database/links.json").unwrap();
    contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    v = serde_json::from_str(&mut contents).unwrap();

    let mut link = Links
    {
        vcredist: String::from(""),
        touchpad: String::from(""),
        touchscreen: String::from(""),
        ec: String::from(""),
        wilco_ec: String::from(""),
        cr50: String::from(""),
        maxim989090: String::from(""),
        ryzen3000audio: String::from(""),
        i2c_link: String::from(""),
        alc5645: String::from(""),
        cyan_audio: String::from(""),
        creative_audio: String::from(""),
        r11_audio: String::from(""),
        drallion_audio: String::from(""),
        ax211_wifi: String::from(""),
        broadwell_rapid_storage: String::from(""),
        cometlake_rapid_storage: String::from(""),
        xe_graphics: String::from(""),
        jasperlake_chipset: String::from(""),
        amd_chipset: String::from(""),
        amd_graphics: String::from(""),
        intel_chipset: String::from(""),
        chrultrabook_tools: String::from(""),
        purchase_driver_portal: String::from("")
    };
    objects = v.as_array().unwrap().clone();

    link.vcredist = helper::remove_quotes(objects[0]["vcredist"].to_string());
    link.touchpad = helper::remove_quotes(objects[0]["touchpad"].to_string());
    link.touchscreen = helper::remove_quotes(objects[0]["touchscreen"].to_string());
    link.ec = helper::remove_quotes(objects[0]["EC"].to_string());
    link.wilco_ec = helper::remove_quotes(objects[0]["Wilco_EC"].to_string());
    link.cr50 = helper::remove_quotes(objects[0]["CR50"].to_string());
    link.maxim989090 = helper::remove_quotes(objects[0]["Maxim989090"].to_string());
    link.ryzen3000audio = helper::remove_quotes(objects[0]["Ryzen3000Audio"].to_string());
    link.i2c_link = helper::remove_quotes(objects[0]["I2C_Link"].to_string());
    link.alc5645 = helper::remove_quotes(objects[0]["ALC5645"].to_string());
    link.cyan_audio = helper::remove_quotes(objects[0]["Cyan_Audio"].to_string());
    link.creative_audio = helper::remove_quotes(objects[0]["Creative_Audio"].to_string());
    link.r11_audio = helper::remove_quotes(objects[0]["R11_Audio"].to_string());
    link.drallion_audio = helper::remove_quotes(objects[0]["Drallion_Audio"].to_string());
    link.ax211_wifi =helper::remove_quotes( objects[0]["AX211_Wifi"].to_string());
    link.broadwell_rapid_storage =helper::remove_quotes( objects[0]["Broadwell_Rapid_Storage"].to_string());
    link.cometlake_rapid_storage = helper::remove_quotes(objects[0]["Cometlake_Rapid_Storage"].to_string());
    link.xe_graphics = helper::remove_quotes(objects[0]["XE_Graphics"].to_string());
    link.jasperlake_chipset = helper::remove_quotes(objects[0]["Jasperlake_Chipset"].to_string());
    link.amd_chipset = helper::remove_quotes(objects[0]["AMD_Chipset"].to_string());
    link.amd_graphics = helper::remove_quotes(objects[0]["AMD_Graphics"].to_string());
    link.intel_chipset = helper::remove_quotes(objects[0]["Intel_Chipset_PS1"].to_string());
    link.chrultrabook_tools = helper::remove_quotes(objects[0]["Chrultrabook_Tools"].to_string());
    link.purchase_driver_portal = helper::remove_quotes(objects[0]["Purchase_Driver_Portal"].to_string());


    let hwid: Vec<String> = helper::get_hwid(); //physical device hardware id (elan0001)
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
        Ok(true) => download_vector.push(&link.vcredist as &str),
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
        Ok(true) => download_vector.push(&link.touchpad as &str),
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
        Ok(true) => download_vector.push(&link.ec as &str),
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
            Ok(true) => download_vector.push(&link.touchscreen),
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
            Ok(true) => download_vector.push(&link.maxim989090),
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
            Ok(true) => download_vector.push(&link.i2c_link),
            Ok(false) => {}
            Err(_) => {
                println!("An Error has occured please try again");
                exit(0)
            }
        }
        let creative_audio = Link::new("Creative Audio Driver download link", &link.creative_audio);
        println!("Due to Legal Constraints, Please download the Creative Audio Driver and move it to C:/oneclickdriverinstalltemp \n\n{}", creative_audio);
    }

    if chromebooks.avaliable_drivers.contains("alc5645-audio") {
        let alc5645 = Confirm::new("Download the ALC5465 audio driver?")
            .with_default(true)
            .prompt();

        match alc5645 {
            Ok(true) => download_vector.push(&link.alc5645),
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
            Ok(true) => download_vector.push(&link.r11_audio),
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
            &link.broadwell_rapid_storage,
        );
        println!("Due to Legal Constraints, Please download the Rapid Storage driver and move it to C:/oneclickdriverinstalltemp. Although not necessary, Intel's version is specialized and provides better battery life. \n\n{}", broadwell_rapid_storage);
    }
    if chromebooks.avaliable_drivers.contains("AX211-Wifi") {
        let ax211_wifi = Link::new("ax211 wifi download link", &link.ax211_wifi);
        println!("Due to Legal Constraints, Please download the Intel Wifi driver and move it to C:/oneclickdriverinstalltemp \n\n{}", ax211_wifi);
    }
    if chromebooks.avaliable_drivers.contains("XE-Graphics") {
        let xe = Link::new("graphics driver download link", &link.xe_graphics);
        println!("Due to Legal Constraints, Please download the graphics driver and move it to C:/oneclickdriverinstalltemp \n\n{}", xe);
    }
    if chromebooks.avaliable_drivers.contains("CezanneChipset")
        || chromebooks.avaliable_drivers.contains("MendocinoChipset")
        || chromebooks
            .avaliable_drivers
            .contains("picasso/dalichipset")
    {
        let amd = Link::new("AMD Chipset Drivers download link", &link.amd_chipset);
        println!("Due to Legal Constraints, Please download the AMD Chipset Drivers and move it to C:/oneclickdriverinstalltemp \n\n{}", amd);
    }
    if chromebooks.avaliable_drivers.contains("Radeon-Graphics")
        || chromebooks.avaliable_drivers.contains("Radeon-GPU")
        || chromebooks.avaliable_drivers.contains("vegagpu")
    {
        let graphics = Link::new("Amd Graphics Driver download link", &link.amd_chipset);
        println!("Due to Legal Constraints, Please download the AMD Graphics Drivers and move it to C:/oneclickdriverinstalltemp \n\n{}", graphics);
    }
    if chromebooks.avaliable_drivers.contains("hd-graphics") {
        let graphics = Link::new("Graphics Driver download link", &link.jasperlake_chipset);
        println!("Due to Legal Constraints, Please download the Graphics Drivers and move it to C:/oneclickdriverinstalltemp \n\n{}", graphics);
    }
    if chromebooks.avaliable_drivers.contains("CR50") {
        let cr50 = Confirm::new("Download the CR50 driver?")
            .with_default(true)
            .prompt();

        match cr50 {
            Ok(true) => download_vector.push(&link.cr50),
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
            Ok(true) => download_vector.push(&link.ryzen3000audio),
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
        let graphics = Link::new("Rapid Storage download link", &link.cometlake_rapid_storage);
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
                download_vector.push(&link.intel_chipset);
                let _ = fs::create_dir_all("/oneclickdriverinstalltemp/zip");
            }
            Ok(false) => {}
            Err(_) => {
                println!("An Error has occured please try again");
                exit(0)
            }
        }
    }
    if chromebooks.board_name == "Drallion" {
        let audio = Confirm::new("Download the audio driver?")
            .with_default(true)
            .prompt();

        match audio {
            Ok(true) => {
                download_vector.push(&link.drallion_audio);
            }
            Ok(false) => {}
            Err(_) => {
                println!("An Error has occured please try again");
                exit(0)
            }
        }
        download_vector.retain(|f| *f != &link.ec);
        download_vector.push(&link.wilco_ec)
    }
    let tools = Confirm::new("Download Chrultrabook Tools?")
    .with_default(true)
    .prompt();
    match tools {
        Ok(true) => {
            download_vector.push(&link.chrultrabook_tools);
        }
        Ok(false) => {}
        Err(_) => {
            println!("An Error has occured please try again");
            exit(0)
        }
    }

    if chromebooks.avaliable_drivers.contains("cAVS")
        || chromebooks.avaliable_drivers.contains("cSOF")
        || chromebooks.avaliable_drivers.contains("sof")
        || chromebooks.avaliable_drivers.contains("Thunderbolt-4")
        || chromebooks.avaliable_drivers.contains("sof-amd")
    {
        let driver_purchase = Link::new("Store link", &link.purchase_driver_portal);
        println!(
            "Your chromebook has audio or thunderbolt drivers avaliable to be purchased. \n\n{}",
            driver_purchase
        );
    }
   

    


    //downloading section
    return helper::to_vec_string(download_vector);
}

fn start_and_wait(program: &str) -> std::io::Result<ExitStatus> {
    let mut child = Command::new(program).spawn()?;
    child.wait()
}
async fn install(length: u8) {
    let mut programs = Vec::new();
    for i in 0..length {
        programs
            .push("C:\\oneclickdriverinstalltemp\\drivers\\".to_owned() + &i.to_string() + ".exe");
    }

    for program in &programs {
        println!("\nStarting: {}", program);
        let status = start_and_wait(program);
        match status {
            Ok(exit) => println!("{} exited with status: {:?}", program, exit),
            Err(e) => eprintln!("Failed to start {}: {}", program, e),
        }
        sleep(Duration::from_secs(2)); // Delay between starting programs
    }
    if Path::new("C:\\oneclickdriverinstalltemp\\zip").exists() == true {
        let target = PathBuf::from("C:\\oneclickdriverinstalltemp\\zip");
        let archive = PathBuf::from("C:\\oneclickdriverinstalltemp\\zip\\autoinstall-intel.zip");
        zip_extract(&archive, &target).unwrap();
        helper::run_chipset_ps1();
    }
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
                    let vector = setup_installation().await;
                    println!("{:?}", vector);
                    download_relay(vector.clone()).await;
                    let start_install = Confirm::new("Start Installation?")
                        .with_default(true)
                        .prompt();
                    match start_install {
                        Ok(true) => {
                            install(vector.len() as u8).await;
                            println!("\nAll Drivers have been installed.");
                        }
                        Ok(false) => {
                            println!("\nUser has denied the installation of drivers")
                        }
                        Err(_) => {
                            println!("\nAn error has occured please try again.")
                        }
                    }
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
