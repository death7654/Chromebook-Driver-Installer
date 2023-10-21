import "./styles.css";
import { invoke } from '@tauri-apps/api/tauri'
import { Command } from "@tauri-apps/api/shell";

const crosEc = await Command.sidecar("./binaries/crosec.2.0.2-installer", ["/S",]);
const crosTouchPad = await Command.sidecar("./binaries/crostouchpad.4.1.4-installer", ["/S",]);
const crosTouchPadEve = await Command.sidecar("./binaries/crostouchpad.4.1.4-onlyeve-installer", ["/S",]);
const crosTouchScreen = await Command.sidecar("./binaries/crostouchscreen.2.9.4-installer", ["/S",]);
const crosWilcoEc = await Command.sidecar("./binaries/wilcoec.1.0.1-installer", ["/S",]);
const crosVcDist = await Command.sidecar("./binaries/VC_redist.x64");

const boardname = await invoke("get_board_name");
let hwid = await invoke("get_hwid")
hwid = hwid.split('\n');
console.log(hwid[3]);

let processExists = null


const cDist = true;
const EC = true;
const touchPad = true;
let touchscreen = false;
let audio = false;

//installs C++
const vcDist = "VC_redist.x64-x86_64-pc-windows-msvc.exe"

let eC = null;
if(boardname == "Wilco"){
  eC = "wilcoec.1.0.1-installer-x86_64-pc-windows-msvc.exe"
}
else{
  eC = "crosec.2.0.2-installer-x86_64-pc-windows-msvc.exe"
}
let touchPadInstaller;
if(boardname == "Eve"){
  touchPadInstaller = "crostouchpad.4.1.4-onlyeve-installer-x86_64-pc-windows-msvc.exe"
}
else
{
  touchPadInstaller = "crostouchpad.4.1.4-installer-x86_64-pc-windows-msvc.exe"
}
const screenTouch = "crostouchscreen.2.9.4-installer-x86_64-pc-windows-msvc.exe"
const Audio = "audio"


let index = 0;
let processName = [vcDist, eC, touchPadInstaller,screenTouch,Audio]
let processStatus = 0;
let process;

function installStatus(){
  if(processExists == null)
  {
    process = processName[0];
  }
  else if (processExists == false && processStatus == 0)
  {
    process = processName[index]
  }
  else if(processExists == true && processStatus == 0){
    process = processName[index]
    console.log(process);
    processStatus++
  }
  else if (processExists == true && processStatus == 1)
  {
    if(processStatus < 2 || processStatus == 1){
    processStatus++
    }
    console.log("running")
  }
  else if (processExists == false && processStatus == 2) {
    index++
    processStatus = 0
    console.log("closed")
  }
}

setInterval(async () => {
  console.log(processExists);
  installStatus();
  processExists = await invoke("check_process",{value1: process} );
},1000)
setTimeout(logProcess,2000);