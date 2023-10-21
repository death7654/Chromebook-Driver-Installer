import "./styles.css";
import { invoke } from "@tauri-apps/api/tauri";
import { Command } from "@tauri-apps/api/shell";

//sets up sidecars, ready to execute

const crosVcDist = await Command.sidecar("./binaries/VC_redist.x64",["/quiet"]);
const crosEc = await Command.sidecar("./binaries/crosec.2.0.2-installer", [
  "/S",
]);
const crosTouchPad = await Command.sidecar(
  "./binaries/crostouchpad.4.1.4-installer",
  ["/S"]
);
const crosTouchPadEve = await Command.sidecar(
  "./binaries/crostouchpad.4.1.4-onlyeve-installer",
  ["/S"]
);
const crosTouchScreen = await Command.sidecar(
  "./binaries/crostouchscreen.2.9.4-installer",
  ["/S"]
);
const crosWilcoEc = await Command.sidecar(
  "./binaries/wilcoec.1.0.1-installer",
  ["/S"]
);
const maxAudio = await Command.sidecar("./binaries/max98090.1.0.4-installer",["/S"]);
const maxAudioR11 = await Command.sidecar('./binaries/max98090-r11.1.0.0-installer',["/S"])
const audioSamus = await Command.sidecar('./binaries/csaudiosstcatpt.1.0.1-installer',["/S"]);

//gets boardname
const boardname = await invoke("get_board_name");
let hwid = await invoke("get_hwid");
console.log(hwid);

let processExists = null;
let touchscreen = false;
let max989090 = false;
let samusAudio = false;

//hardware ids identified in driver .infs
const touchScreenHWID = ["ACPI\\ATML0001", "ACPI\\ELAN0001","ACPI\\MLFS0000","ACPI\\RAYD0001"]
const max989090HWID = ["ACPI\\VEN_193C&DEV_9890&REV_0002","ACPI\\193C9890"]

//checks if touchscreen is avaliable
const touchScreenAvaliable = touchScreenHWID.some(element => {
  return hwid.includes(element);
});
if (touchScreenAvaliable == true)
{
  touchscreen = true;
}
//checks if there is a max989090 in both cyan and other chromebooks that have this
const isMax989090 = max989090HWID.some(element => {
  return hwid.includes(element);
});

if(isMax989090 == true)
{
  max989090 == true;
}

if(boardname == "Samus")
{
  samusAudio == true;
}


//Assigns variables to file name to check if process has started
const vcDist = "VC_redist.x64-x86_64-pc-windows-msvc.exe";

let ECFile = null;
if (boardname == "Wilco") {
  ECFile = "wilcoec.1.0.1-installer-x86_64-pc-windows-msvc.exe";
} else {
  ECFile = "crosec.2.0.2-installer-x86_64-pc-windows-msvc.exe";
}

let touchPadInstaller;
if (boardname == "Eve") {
  touchPadInstaller =
    "crostouchpad.4.1.4-onlyeve-installer-x86_64-pc-windows-msvc.exe";
} else {
  touchPadInstaller = "crostouchpad.4.1.4-installer-x86_64-pc-windows-msvc.exe";
}

const touchScreenInstaller =
  "crostouchscreen.2.9.4-installer-x86_64-pc-windows-msvc.exe";

const Audio = "audio";

let index = 0;
let processName = [
  vcDist,
  ECFile,
  touchPadInstaller,
  touchScreenInstaller,
  Audio,
];
let processStatus = 0;
let process;

//cycles through all drivers in order, the computer of this app
function installStatus() {
  if (processExists == null) {
    process = processName[0];

  } else if (processExists == false && processStatus == 0) {
    process = processName[index];
    installDrivers(process);

  } else if (processExists == true && processStatus == 0) {
    process = processName[index];
    //console.log(process);
    processStatus++;

  } else if (processExists == true && processStatus == 1) {
    if (processStatus < 2 || processStatus == 1) {
      processStatus++;
    }
    console.log("running");
  } else if (processExists == false && processStatus == 2) {
    index++;
    processStatus = 0;
    //console.log("process-closed");
  }
}

//sets interval
function startInstall() {
setInterval(async () => {
  installStatus();
  processExists = await invoke("check_process", { value1: process });
}, 1000);
}


//executes sidecars in order
async function installDrivers(driver) {
  console.log(driver);
  if (driver == vcDist) {
    crosVcDist.execute();
  } else if (driver == ECFile) {
    if (driver == "wilcoec.1.0.1-installer-x86_64-pc-windows-msvc.exe") {
      crosWilcoEc.execute();
    } else {
      crosEc.execute();
    }
  } else if (driver == touchPadInstaller) {
    if (
      driver ==
      "crostouchpad.4.1.4-onlyeve-installer-x86_64-pc-windows-msvc.exe"
    ) {
      crosTouchPadEve.execute();
    } else {
      crosTouchPad.execute();
    }
  } else if (touchscreen == true && driver == touchScreenInstaller) {
    crosTouchScreen.execute();
  }else if (touchscreen == false && driver == touchScreenInstaller) {
    index++;
  }
   else if (max989090 == true && driver == Audio) {
    if(boardname == "Cyan")
    {
      maxAudioR11.execute();
    }
    else{
      maxAudio.execute();
    }
  }
  else if (samusAudio == true && driver == Audio){
    audioSamus.execute();
  }
}
