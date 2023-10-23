//THIS WILL NOT WORK IN DEBUG DUE TO THE FILE IN BINARYIES HAVING THE x86_64 EXTENSION AND THE BUILD HAVING NONE

import "./styles.css";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";

//sets variables so updates are easily done
const touchScreenDriver = "https://github.com/coolstar/driverinstallers/raw/master/crostouchscreen/crostouchscreen.2.9.4-installer.exe";
const wilcoecDriver = "https://github.com/coolstar/driverinstallers/raw/master/wilcoec/wilcoec.1.0.1-installer.exe";
const ecDriver = "https://github.com/coolstar/driverinstallers/raw/master/crosec/crosec.2.0.2-installer.exe";
const samusAudioDriver = "https://github.com/coolstar/driverinstallers/raw/master/csaudiosstcatpt/csaudiosstcatpt.1.0.1-installer.exe";
const maxr11AudioDriver = "https://github.com/coolstar/driverinstallers/raw/master/max98090-r11/max98090-r11.1.0.0-installer.exe";
const max98090driver = "https://github.com/coolstar/driverinstallers/raw/master/max98090/max98090.1.0.4-installer.exe";
const realTekAudioDriver ="https://coolstar.org/chromebook/downloads/drivers/alc5645%20audio.exe";
const i2sDriverLink = "https://coolstar.org/chromebook/downloads/drivers/BayTrailChipsetDriver-Lenovo.exe";
const drallionAudio = ""
document.addEventListener("contextmenu", (event) => event.preventDefault());

setTimeout(async() => {
  invoke('on_start')
  invoke('downloader');
})

//gets boardname
let boardname;
let hwid;  
setTimeout(async () => {
  boardname = await invoke("get_board_name");
  hwid = await invoke("get_hwid");
});
let processExists = null;
let samusAudio = false;

console.log(boardname)

//hardware ids identified in driver .infs
const touchScreenHWID = [
  "ACPI\\ATML0001",
  "ACPI\\ELAN0001",
  "ACPI\\MLFS0000",
  "ACPI\\RAYD0001",
];
const max989090HWID = ["ACPI\\VEN_193C&DEV_9890&REV_0002", "ACPI\\193C9890"];
const i2sDriverBoardname = [
  "Winky",
  "Swanky",
  "Squawks",
  "Quawks",
  "Orco",
  "Kip",
  "Heli",
  "Gnawty",
  "Glimmer",
  "Engaurde",
  "Clapper",
  "Candy",
  "Banjo",
];
const realTekDriver = [
  "Banon",
  "Edgar",
  "Kefka",
  "Reks",
  "Relm",
  "Setzer",
  "Terra",
  "Terra13",
  "Ultima",
  "Wizpig",
  "Celes",
];
let touchscreen;
let max989090;
let i2sDriver;
let realTekAudio;

//starts install function
function startInstall() {
  //allows items to be checked and marked as true since await cannot be used outside a function
  setInterval(async () => {
    installStatus();
    if(process !== undefined)
    {
    processExists = await invoke("check_process", { value1: process });
    }
  }, 1000);
  //checks if touchscreen is avaliable
 touchscreen = touchScreenHWID.some((element) => {
  return hwid.includes(element);
});

//checks if there is a max989090 in both cyan and other chromebooks that have this
 max989090 = max989090HWID.some((element) => {
  return hwid.includes(element);
});

//checks if there is a need for the i2s driver
i2sDriver = i2sDriverBoardname.some((element) => {
  return boardname.includes(element);
});
//checks if there is a need for the realtek driver
realTekAudio = realTekDriver.some((element) => {
  return boardname.includes(element);
});

if (boardname === "Samus") {
  samusAudio = true;
}
  document.getElementById('mainPage').style.display = "none"
  document.getElementById('progressPage').style.display = "block"
}

//Assigns variables to file name to check if process has started, and if it is finished
const vcDist = "vc_redist.x64.exe";

let ECFile = null;
if (boardname === "Wilco") {
  ECFile = "wilcoec.1.0.1-installer.exe";
  invoke("download_driver", {value1: wilcoecDriver})
} else {
  ECFile = "crosec.2.0.2-installer.exe";
  invoke("download_driver", {value1: ecDriver})

}

const touchPadInstaller = "crostouchpad.4.1.4-installer.exe";

const touchScreenInstaller ="crostouchscreen.2.9.4-installer.exe";
if (touchscreen === true)
{
  invoke("download_driver", {value1: touchScreenDriver})
}

//assigns process name
let Audio;
if (max989090 === true) {
  if (boardname === "Cyan") {
    Audio = "max98090-r11.1.0.0-installer.exe";
    invoke('download_driver',{value1: maxr11AudioDriver})
  } else {
    Audio = "max98090.1.0.4-installer.exe";
    invoke('download_driver',{value1: max98090driver});
  }
} else if (boardname === "Samus") {
  Audio = "csaudiosstcatpt.1.0.1-installer.exe";
  invoke('download_driver',{value1: samusAudioDriver});
} else if (realTekAudio === true) {
  Audio = "alc5645audio.exe";
  invoke('download_driver',{value1: realTekAudioDriver});
} else if (boardname === "Drallion")
{
  Audio = "Realtek-High-Definition-Audio-Driver_266V7_WIN_6.0.9341.1_A13.EXE"
}
const i2sInstaller = "BayTrailChipsetDriver-Lenovo.exe";
if (i2sDriver === true)
{
  invoke('download_driver',{value1: i2sDriverLink});
}

//essentially only allows one process to execute at one time
let index = 0;
let processName = [
  vcDist,
  ECFile,
  touchPadInstaller,
  touchScreenInstaller,
  i2sInstaller,
  Audio,
];
let processStatus = 0;
let process;

//cycles through all drivers in order on cs' website
function installStatus() {
  if (processExists == null) {
    process = processName[0];
  } else if (processExists === false && processStatus === 0) {
    process = processName[index];
    installDrivers(process);
    document.getElementById('process').innerText = process;
    console.log(process)
  } else if (processExists === true && processStatus === 0) {
    process = processName[index];
    //console.log(process);
    document.getElementById('process').innerText = process;
    processStatus++;
  } else if (processExists === true && processStatus === 1) {
    if (processStatus < 2 || processStatus === 1) {
      processStatus++;
    }
    console.log("running");
  } else if (processExists === false && processStatus === 2) {
    index++;
    processStatus = 0;
    //console.log("process-closed");
  }
  //sometimes the process exists faster than js can change status
  else if(processExists === false && processStatus === 1)
  {
    index++
    processStatus = 0;
  }
}

function installFinished(){
  document.getElementById('progressPage').style.display = "none"
  document.getElementById('finished').style.display = "block"

}

//executes sidecars in order
async function installDrivers(driver) {
  console.log(driver);
  if (driver === vcDist) 
  {
    setTimeout(async() => {invoke('install_vc_dist');})
  } 
  else if (driver === ECFile) 
  {
    if (driver === "wilcoec.1.0.1-installer.exe") 
    {
      setTimeout(async() => {invoke('install',{value1:"C:\\tmp\\wilcoec.1.0.1-installer.exe"});})
    } 
    else 
    {
      setTimeout(async() => {invoke('install',{value1:"C:\\tmp\\crosec.2.0.2-installer.exe"});})
    }
  } 
  else if (driver === touchPadInstaller) 
  {
    setTimeout(async() => {invoke('install',{value1:"C:\\tmp\\crostouchpad.4.1.4-installer.exe"});})
  } 
  else if (touchscreen === true && driver === touchScreenInstaller) 
  {
    setTimeout(async() => {invoke('install',{value1:"C:\\tmp\\crostouchscreen.2.9.4-installer.exe"});})
  } 
  else if (touchscreen === false && driver === touchScreenInstaller) {
    index++;
  } 
  else if (i2sDriver === true && driver === i2sInstaller) {
    setTimeout(async() => {invoke('install',{value1:"C:\\tmp\\BayTrailChipsetDriver-Lenovo.exe"});})
  } 
  else if (i2sDriver === false && driver === i2sInstaller) {
    index++;
  } 
  else if (max989090 === true && driver === Audio) {
    if (boardname === "Cyan") {
      setTimeout(async() => {invoke('install',{value1:"C:\\tmp\\max98090-r11.1.0.0-installer.exe"});})
    } else {
      setTimeout(async() => {invoke('install',{value1:"C:\\tmp\\max98090.1.0.4-installer.exe"});})
    }
  } 
  else if (samusAudio === true && driver === Audio) {
    setTimeout(async() => {invoke('install',{value1:"C:\\tmp\\csaudiosstcatpt.1.0.1-installer.exe"});})
  } 
  else if (realTekAudio === true && driver === Audio) {
    setTimeout(async() => {invoke('install',{value1:"C:\\tmp\\alc5645 audio.exe"});})
  }
  else if (boardname === "Drallion" && driver === Audio) {
    setTimeout(async() => {invoke('install',{value1:"C:\\tmp\\Realtek-High-Definition-Audio-Driver_266V7_WIN_6.0.9341.1_A13.EXE"});})
  }
  else if(driver === undefined) {
    installFinished();
  }
}

//document.getElementById("install").addEventListener('keydown',startInstall());
document.getElementById("install").addEventListener('mousedown',() => {startInstall()});
document.getElementById("close").addEventListener('mousedown',() => {appWindow.close()});
document.getElementById("close1").addEventListener('mousedown',() => {appWindow.close()});


