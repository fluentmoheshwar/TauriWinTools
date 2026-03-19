const { invoke } = window.__TAURI__.core;

export async function run_process(program, args, elevated) {
  try {
    await invoke("run_process", {
      program,
      args,
      elevated,
    });
  } catch (error) {
    window.alert(`Error: ${error}`);
  }
}

export async function fKeySender() {
  try {
    const url =
      "https://api.github.com/repos/ThioJoe/F-Key-Sender/releases/latest";
    const response = await fetch(url);
    const data = await response.json();
    const downloadUrl = data.assets[0].browser_download_url;
    run_process(
      "cmd.exe",
      `/c powershell -Command Start-BitsTransfer -Source ${downloadUrl} -Destination $env:TEMP && %TEMP%\\F_Key_Sender.exe`,
      false,
    );
  } catch (error) {
    window.alert(`Error fetching F-Key Sender download link: ${error}`);
  }
}

window.fKeySender = fKeySender;
window.run_process = run_process;
