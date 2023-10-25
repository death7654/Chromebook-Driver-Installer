
import "./styles.css";
import { invoke } from "@tauri-apps/api/tauri";

//Assigns variables to file name to check if process has started, and if it is finished
const vcDist = "vc_redist.x64.exe";
const ECFile = "crosec.2.0.2-installer.exe";
const touchPadInstaller = "crostouchpad.4.1.4-installer.exe";
let touchScreenInstaller;
let Audio;
const i2sInstaller = "BayTrailChipsetDriver-Lenovo.exe";


//creates download folder
setTimeout(async () => {invoke("on_start");});

//prevents rightclick
document.addEventListener("contextmenu", (event) => event.preventDefault());

//gets boardname
let boardname;
let hwid;
setTimeout(async () => {
  boardname = await invoke("get_board_name");
  hwid = await invoke("get_hwid");
});
let processExists = null;
let samusAudio = false;

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
let downloading;

let index;
let processName;
let processStatus;
let process;

//starts install function
function startDownload() {
  setTimeout(async () => {
    invoke("download_standard");
  });
  let filesDownloaded;
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

  downloadDriver();

  let files = 3;
  if (touchscreen === true)
  {
    files++
  }
  if(max989090 === true)
  {
    files++
  }
  if(i2sDriver === true)
  {
    files++
  }
  if(realTekAudio === true)
  {
    files++
  }

  downloading = setInterval(async () => {
    filesDownloaded = await invoke("file_check");
    filesDownloaded = parseInt(filesDownloaded);
    if(files === filesDownloaded)
  {
    startInstall();
    console.log('completed');
    document.getElementById("installingtext").innerText = "Installing";
  }
  },2000);
  document.getElementById("mainPage").style.display = "none";
  document.getElementById("progressPage").style.display = "block";
  document.getElementById("installingtext").innerText = "Downloading Please Wait"
}

function downloadDriver() {
  console.log(touchscreen)
  if (touchscreen == true) {
    invoke("download_touchscreen");
    touchScreenInstaller = "crostouchscreen.2.9.4-installer.exe"
  }
  if (max989090 == true) {
    if (boardname === "Cyan") {
      Audio = "max98090-r11.1.0.0-installer.exe";
      invoke("download_cyan");
    } else {
      Audio = "max98090.1.0.4-installer.exe";
      invoke("download_max98980");
    }
  }
  if (boardname == "Samus") {
    Audio = "csaudiosstcatpt.1.0.1-installer.exe";
    invoke("download_samus_audio");
    samusAudio = true;
  }
  if (realTekAudio == true) {
    Audio = "alc5645audio.exe";
    invoke("download_realtek");
  }
  if (i2sDriver == true) {
    invoke("download_i2s");
  }

  //sets up variables for install
  index = 0;
processName = [
  vcDist,
  ECFile,
  touchPadInstaller,
  touchScreenInstaller,
  i2sInstaller,
  Audio,
];
processStatus = 0;
process;

}
function startInstall(){
  installStatus();
  setInterval(async () => {
    if (process !== undefined) {
      processExists = await invoke("check_process", { value1: process });
      installStatus();
    }
  }, 1000);
  clearInterval(downloading);
}

//cycles through all drivers in order on cs' website
function installStatus() {
  if (processExists === null) {
    process = processName[0];
  } else if (processExists === false && processStatus === 0) {
    process = processName[index];
    installDrivers(process);
    document.getElementById("installingtext").innerText = "Installing" + " " + process;
    console.log(process);
  } else if (processExists === true && processStatus === 0) {
    process = processName[index];
    document.getElementById("installingtext").innerText = "Installing" + " " + process;
    processStatus++;
  } else if (processExists === true && processStatus === 1) {
    if (processStatus < 2 || processStatus === 1) {
      processStatus++;
    }
    console.log("running");
  } else if (processExists === false && processStatus === 2) {
    index++;
    processStatus = 0;
    console.log("process-closed");
  }
  //sometimes the process exists faster than js can change status
  else if (processExists === false && processStatus === 1) {
    index++;
    processStatus = 0;
  }
}

function installFinished() {
  document.getElementById("progressPage").style.display = "none";
  document.getElementById("finished").style.display = "block";
}

//executes sidecars in order
async function installDrivers(driver) {
  if (driver === vcDist) {
    setTimeout(async () => {
      invoke("install_vc_dist");
    });
  } else if (driver === ECFile) {
    if (driver === "crosec.2.0.2-installer.exe") {
      setTimeout(async () => {
        invoke("install", { value1: "C:\\oneclicktmp\\crosec.2.0.2-installer.exe" });
      });
    }
  } else if (driver === touchPadInstaller) {
    setTimeout(async () => {
      invoke("install", {
        value1: "C:\\oneclicktmp\\crostouchpad.4.1.4-installer.exe",
      });
    });
  } else if (touchscreen === true && driver === touchScreenInstaller) {
    setTimeout(async () => {
      invoke("install", {
        value1: "C:\\oneclicktmp\\crostouchscreen.2.9.4-installer.exe",
      });
    });
  } else if (touchscreen === false && driver === touchScreenInstaller) {
    index++;
  } else if (i2sDriver === true && driver === i2sInstaller) {
    setTimeout(async () => {
      invoke("install", {
        value1: "C:\\oneclicktmp\\BayTrailChipsetDriver-Lenovo.exe",
      });
    });
  } else if (i2sDriver === false && driver === i2sInstaller) {
    index++;
  } else if (max989090 === true && driver === Audio) {
    if (boardname === "Cyan") {
      setTimeout(async () => {
        invoke("install", {
          value1: "C:\\oneclicktmp\\max98090-r11.1.0.0-installer.exe",
        });
      });
    } else {
      setTimeout(async () => {
        invoke("install", { value1: "C:\\oneclicktmp\\max98090.1.0.4-installer.exe" });
      });
    }
  } else if (samusAudio === true && driver === Audio) {
    setTimeout(async () => {
      invoke("install", {
        value1: "C:\\oneclicktmp\\csaudiosstcatpt.1.0.1-installer.exe",
      });
    });
  } else if (realTekAudio === true && driver === Audio) {
    setTimeout(async () => {
      invoke("install", { value1: "C:\\oneclicktmp\\alc5645 audio.exe" });
    });
  } else if (driver === undefined) {
    installFinished();
  }
}

//document.getElementById("install").addEventListener('keydown',startInstall());
document.getElementById("install").addEventListener("mousedown", () => {
  startDownload();
});
document.getElementById("close").addEventListener("mousedown", () => {
  invoke("delete_dir");
});
document.getElementById("close1").addEventListener("mousedown", () => {
  invoke("delete_dir");
});
