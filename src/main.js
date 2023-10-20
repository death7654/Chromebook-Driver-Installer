import "./styles.css";
import { invoke } from '@tauri-apps/api/tauri'
import { Command } from "@tauri-apps/api/shell";

const ec = await Command.sidecar("./binaries/crosec.2.0.2-installer", ["/S",]);
const touchPad = await Command.sidecar("./binaries/crostouchpad.4.1.4-installer", ["/S",]);
const touchPadEve = await Command.sidecar("./binaries/crostouchpad.4.1.4-onlyeve-installer", ["/S",]);
const touchScreen = await Command.sidecar("./binaries/crostouchscreen.2.9.4-installer", ["/S",]);
const wilcoEc = await Command.sidecar("./binaries/wilcoec.1.0.1-installer", ["/S",]);
const boardname = await invoke("get_board_name");


setInterval(async () => {
  let processExists = await invoke("check_process");
  console.log(processExists)
},1000)

function installDrivers() {
  console.log(boardname);
/*
  if (boardname !== "Eve") {
    touchPad.execute();
    console.log("installing touchpad");

  }
  else{
    touchPadEve.execute();
    console.log("installing eve touchpad");
  }

  if (boardname == "Wilco")
  {
    ec.execute();
  }
  else
  {
    wilcoEc.execute();
    console.log("installing touchpad");
  }
*/
  touchScreen.execute();
  console.log("installing touchscreen");


}

installDrivers();
