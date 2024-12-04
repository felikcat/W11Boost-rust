use std::error::Error;
use winsafe::{prelude::advapi_Hkey, HKEY};
use crate::common::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    let hklm = HKEY::LOCAL_MACHINE;
    let hkcu = HKEY::CURRENT_USER;

    // If allowed (1): unused apps would be uninstalled with their user data left intact, then reinstalled if launched afterwards at any point in time.
    set_dword(&hklm, r"SOFTWARE\Policies\Microsoft\Windows\Appx", "AllowAutomaticAppArchiving", 0)?;

    // Make all users opted out of the Windows Customer Experience Improvement Program.
    set_dword(&hklm, r"SOFTWARE\Policies\Microsoft\SQMClient\Windows", "CEIPEnable", 0)?;

     // Shows what's slowing down bootups and shutdowns.
     set_dword(
        &hklm,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
        "verbosestatus",
        1,
    )?;

    // Ask to not allow execution of experiments by Microsoft.
    set_dword(
        &hklm,
        r"SOFTWARE\Microsoft\PolicyManager\current\device\System",
        "AllowExperimentation",
        0,
    )?;

    // Power Throttling causes severe performance reduction for VMWare.
    // Workstation 17
    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling",
        "PowerThrottlingOff",
        1,
    )?;

    // https://docs.microsoft.com/en-us/windows/desktop/win7appqual/fault-tolerant-heap
    // FTH being enabled causes issues with specific apps such as Assetto Corsa.
    set_dword(
        &hklm,
        r"SOFTWARE\Microsoft\FTH",
        "Enabled",
        0,
    )?;

    // Automated file cleanup without user interaction is a bad idea, even if ran only on low-disk space events.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\Appx",
        "AllowStorageSenseGlobal",
        0,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\StorageSense",
        "AllowStorageSenseGlobal",
        0,
    )?;
    remove_subkey(&hklm, r"SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense")?;
    
    // Allocate more RAM to NTFS' paged pool.
    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Policies",
        "NtfsForceNonPagedPoolAllocation",
        1,
    )?;
    std::process::Command::new("fsutil.exe")
        .args(["behavior", "set", "memoryusage", "2"])
        .output()?;

    // Disable automatic repair to instead ask for a repair.
    // Does not disable Windows' Recovery environment thankfully.
    std::process::Command::new("bcdedit.exe")
        .args(["/set", "{default}", "recoveryenabled", "no"])
        .output()?;

    // Do not page drivers and other system code to a disk, keep it in memory.
    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "DisablePagingExecutive",
        1,
    )?;

    Ok(())
}