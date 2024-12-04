pub mod common;
mod gui;
use fltk::dialog;
use gui::draw_gui;
use windows_sys::Win32::Security::{DuplicateTokenEx, SecurityImpersonation, TokenImpersonation, SECURITY_ATTRIBUTES, TOKEN_ALL_ACCESS, TOKEN_DUPLICATE};
use winsafe::msg::wm::Close;
use std::error::Error;
use std::ptr::null_mut;
use std::time::Duration;
use std::{mem, thread};
use windows::Win32::System::Services::{
    CloseServiceHandle, OpenSCManagerW, OpenServiceW, QueryServiceStatus, QueryServiceStatusEx, StartServiceW, SC_HANDLE, SC_STATUS_PROCESS_INFO, SERVICE_RUNNING, SERVICE_STATUS_PROCESS, SERVICE_STOPPED
};
use windows::core::w;
use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows_sys::Win32::System::SystemServices::MAXIMUM_ALLOWED;
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcess};
use windows_sys::Win32::System::Threading::{
    CREATE_UNICODE_ENVIRONMENT, CreateProcessWithTokenW, LOGON_WITH_PROFILE, OpenProcessToken,
    PROCESS_INFORMATION, STARTUPINFOW,
};
use winsafe::{self as ws, co, prelude::*};

fn create_access_token_from_pid(process_id: u32) {
    let mut dup_token = INVALID_HANDLE_VALUE;
    unsafe { 
    let process = OpenProcess(MAXIMUM_ALLOWED, 0, process_id);
    if !process.is_null() {
        let mut token = INVALID_HANDLE_VALUE;
        OpenProcessToken(process, TOKEN_DUPLICATE, &mut token);

        let attributes = SECURITY_ATTRIBUTES {
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: null_mut(),
            bInheritHandle: 0,
        };

        DuplicateTokenEx(token, TOKEN_ALL_ACCESS, &attributes, SecurityImpersonation, TokenImpersonation, &mut dup_token);
    }
};
}

fn start_trusted_installer_service() -> Result<u32, Box<dyn Error>> {
    const SLEEP_INTERVAL: u64 = 100;
    const MAX_ATTEMPTS: u32 = 50; // 50 * 100ms = 5 seconds

    let scm_handle = unsafe {
        OpenSCManagerW(
            None,
            None,
            windows::Win32::System::Services::SC_MANAGER_CONNECT,
        )
    }
    .expect("Failed to connect to Service Control Manager");

    let service_handle = unsafe {
        OpenServiceW(
            scm_handle,
            w!("TrustedInstaller"),
            windows::Win32::System::Services::SERVICE_START
                | windows::Win32::System::Services::SERVICE_QUERY_STATUS,
        )
    }
    .expect("Failed to open TrustedInstaller service");

    let mut attempts = MAX_ATTEMPTS;
    let mut process_id = 0;

    while attempts > 0 {
        let status = SERVICE_STATUS_PROCESS::default();

        if status.dwCurrentState == SERVICE_RUNNING {
            process_id = status.dwProcessId;
            break;
        }

        if status.dwCurrentState == SERVICE_STOPPED {
            unsafe { StartServiceW(service_handle, None) }
                .expect("Failed to start TrustedInstaller service");
        }

        thread::sleep(Duration::from_millis(SLEEP_INTERVAL));
        attempts -= 1;
    }

    // Cleanup.
    unsafe { 
        CloseServiceHandle(service_handle).expect("Failed to close service_handle");
        CloseServiceHandle(scm_handle).expect("Failed to close scm_handle");
    }

    Ok(process_id)
}

fn main() -> Result<(), Box<dyn Error>> {
    let service_id = start_trusted_installer_service().unwrap();
    create_access_token_from_pid(service_id);

    let mut process = ws::HPROCESSLIST::CreateToolhelp32Snapshot(
        co::TH32CS::SNAPPROCESS,
        Some(ws::GetCurrentProcessId()),
    )?;

    let mut pe32 = ws::PROCESSENTRY32::default();
    process.Process32First(&mut pe32)?;

    unsafe {
        let process_handle = GetCurrentProcess();
        let mut token_handle = INVALID_HANDLE_VALUE;

        let startup_info = STARTUPINFOW {
            cb: std::mem::size_of::<STARTUPINFOW>() as u32,
            ..mem::zeroed()
        };
        let mut process_info = PROCESS_INFORMATION { ..mem::zeroed() };

        OpenProcessToken(process_handle, MAXIMUM_ALLOWED, &mut token_handle);

        ;

        let exe_path = widestring::U16CString::from_str(pe32.szExeFile()).unwrap();

        CreateProcessWithTokenW(
            token_handle,
            LOGON_WITH_PROFILE,
            exe_path.as_ptr(),
            null_mut(),
            CREATE_UNICODE_ENVIRONMENT,
            null_mut(),
            null_mut(),
            &startup_info,
            &mut process_info,
        );
    }
    match draw_gui() {
        Ok(_) => println!("draw_gui() exited successfully"),
        Err(e) => dialog::alert(0, 0, &format!("W11Boost -> draw_gui() failed: {}", e)),
    }
    Ok(())
}
