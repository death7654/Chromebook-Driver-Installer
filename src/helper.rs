

pub fn to_vec_string(input: Vec<&str>) -> Vec<String> {
    let strings: Vec<String> = input.iter().map(|&s| s.into()).collect();
    return strings;
}
pub fn remove_quotes(input: String) -> String {
    input[1..(input.len() - 1)].to_string()
}
pub fn get_boardname() -> String {
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
pub fn get_hwid() -> Vec<String> {
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
pub fn run_chipset_ps1()
{
    let _: Result<std::process::Output, std::io::Error> =
        std::process::Command::new("powershell.exe")
            .args(vec![
                "-ExecutionPolicy",
                "bypass",
                "-file",
                "C:\\oneclickdriverinstalltemp\\zip\\autoinstall-intel.ps1",
            ])
            .output();
}
