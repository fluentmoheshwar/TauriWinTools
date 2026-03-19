#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use windows::{
    core::*,
    Win32::Foundation::{CloseHandle},
    Win32::System::Threading::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW, PROCESS_CREATION_FLAGS},
    Win32::UI::Shell::{ShellExecuteExW, SHELLEXECUTEINFOW, SEE_MASK_DEFAULT},
    Win32::UI::WindowsAndMessaging::SW_SHOW,
};

#[tauri::command]
fn run_process(program: String, args: Option<String>, elevated: bool) -> std::result::Result<(), String> {
    unsafe {
        if elevated {
            fn to_wide(s: &str) -> Vec<u16> {
                let mut v: Vec<u16> = s.encode_utf16().collect();
                v.push(0);
                v
            }

            let verb = to_wide("runas");
            let file = to_wide(&program);
            let params = args.as_ref().map(|a| to_wide(a));

            let mut sei = SHELLEXECUTEINFOW::default();
            sei.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
            sei.fMask = SEE_MASK_DEFAULT;
            sei.lpVerb = PCWSTR(verb.as_ptr());
            sei.lpFile = PCWSTR(file.as_ptr());
            if let Some(ref p) = params {
                sei.lpParameters = PCWSTR(p.as_ptr());
            }
            sei.nShow = SW_SHOW.0;

            if ShellExecuteExW(&mut sei).is_ok() {
                Ok(())
            } else {
                Err("Failed to launch elevated process".into())
            }
        } else {
            let mut si = STARTUPINFOW::default();
            let mut pi = PROCESS_INFORMATION::default();

            let cmdline = if let Some(a) = args {
                format!("{} {}", program, a)
            } else {
                program.clone()
            };

            let mut wide: Vec<u16> = cmdline.encode_utf16().collect();
            wide.push(0);

            let success = CreateProcessW(
                None,
                Some(PWSTR(wide.as_mut_ptr())),
                None,
                None,
                false,
                PROCESS_CREATION_FLAGS(0),
                None,
                None,
                &mut si,
                &mut pi,
            );

            if success.is_ok() {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
                Ok(())
            } else {
                Err("Failed to launch normal process".into())
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![run_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
