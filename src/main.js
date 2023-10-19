import "./styles.css";
import { invoke } from '@tauri-apps/api/tauri'
import { Command } from "@tauri-apps/api/shell";

const touchScreen = await Command.sidecar("./binaries/crostouchpad.4.1.4-installer", [
  "/S",
]);
const boardname2 = await invoke("get_board_name");

function installDrivers() {
  touchScreen.execute();
  console.log(boardname2);

}
installDrivers();
