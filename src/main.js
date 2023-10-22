import "./styles.css";
import { invoke } from "@tauri-apps/api/tauri";
import { Command } from "@tauri-apps/api/shell";
import { appWindow } from "@tauri-apps/api/window";

document.addEventListener("contextmenu", (event) => event.preventDefault());

//sets up sidecars, ready to execute

const crosVcDist =  Command.sidecar("./binaries/VC_redist.x64", [
  "/quiet",
]);
const crosEc =  Command.sidecar("./binaries/crosec.2.0.2-installer", [
  "/S",
]);
const crosTouchPad =  Command.sidecar(
  "./binaries/crostouchpad.4.1.4-installer",
  ["/S"]
);
const crosTouchPadEve =  Command.sidecar(
  "./binaries/crostouchpad.4.1.4-onlyeve-installer",
  ["/S"]
);
const crosTouchScreen =  Command.sidecar(
  "./binaries/crostouchscreen.2.9.4-installer",
  ["/S"]
);
const crosWilcoEc =  Command.sidecar(
  "./binaries/wilcoec.1.0.1-installer",
  ["/S"]
);
const maxAudio =  Command.sidecar("./binaries/max98090.1.0.4-installer", [
  "/S",
]);
const maxAudioR11 =  Command.sidecar(
  "./binaries/max98090-r11.1.0.0-installer",
  ["/S"]
);
const audioSamus =  Command.sidecar(
  "./binaries/csaudiosstcatpt.1.0.1-installer",
  ["/S"]
);
/*
const i2s =  Command.sidecar("./binaries/BayTrailChipsetDriver-Lenovo", [
  "/S",
]);
*/
const realTek =  Command.sidecar("./binaries/alc5645audio", ["/S"]);

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
const vcDist = "VC_redist.x64.exe";

let ECFile = null;
if (boardname === "Wilco") {
  ECFile = "wilcoec.1.0.1-installer.exe";
} else {
  ECFile = "crosec.2.0.2-installer.exe";
}

let touchPadInstaller;
if (boardname === "Eve") {
  touchPadInstaller =
    "crostouchpad.4.1.4-onlyeve-installer.exe";
} else {
  touchPadInstaller = "crostouchpad.4.1.4-installer.exe";
}

const touchScreenInstaller =
  "crostouchscreen.2.9.4-installer.exe";

//assigns process name
let Audio;
if (max989090 === true) {
  if (boardname === "Cyan") {
    Audio = "max98090-r11.1.0.0-installer.exe";
  } else {
    Audio = "max98090.1.0.4-installer.exe";
  }
} else if (boardname === "Samus") {
  Audio = "csaudiosstcatpt.1.0.1-installer.exe";
} else if (realTekAudio === true) {
  Audio = "alc5645audio.exe";
}
const i2sInstaller = "BayTrailChipsetDriver-Lenovo.exe";


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
  } else if (processExists === true && processStatus === 0) {
    process = processName[index];
    //console.log(process);
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
}

function installFinished(){
  document.getElementById('progressPage').style.display = "none"
  document.getElementById('finished').style.display = "block"

}

//executes sidecars in order
async function installDrivers(driver) {
  console.log(driver);
  if (driver === vcDist) {
    crosVcDist.execute();
  } else if (driver === ECFile) {
    if (driver === "wilcoec.1.0.1-installer-x86_64-pc-windows-msvc.exe") {
      crosWilcoEc.execute();
    } else {
      crosEc.execute();
    }
  } else if (driver === touchPadInstaller) {
    if (
      driver ===
      "crostouchpad.4.1.4-onlyeve-installer-x86_64-pc-windows-msvc.exe"
    ) {
      crosTouchPadEve.execute();
    } else {
      crosTouchPad.execute();
    }
  } else if (touchscreen === true && driver === touchScreenInstaller) {
    crosTouchScreen.execute();
  } else if (touchscreen === false && driver === touchScreenInstaller) {
    index++;
  } else if (i2sDriver === true && driver === i2sInstaller) {
    i2s.execute();
  } else if (i2sDriver === false && driver === i2sInstaller) {
    index++;
  } else if (max989090 === true && driver === Audio) {
    if (boardname === "Cyan") {
      maxAudioR11.execute();
    } else {
      maxAudio.execute();
    }
  } else if (samusAudio === true && driver === Audio) {
    audioSamus.execute();
  } else if (realTekAudio === true && driver === Audio) {
    realTek.execute();
  }
  else if(driver === undefined) {
    installFinished();
  }
}

//document.getElementById("install").addEventListener('keydown',startInstall());
document.getElementById("install").addEventListener('mousedown',() => {startInstall()});
document.getElementById("close").addEventListener('mousedown',() => {appWindow.close()});
document.getElementById("close1").addEventListener('mousedown',() => {appWindow.close()});


