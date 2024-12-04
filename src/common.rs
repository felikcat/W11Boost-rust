use chrono::{Datelike, Timelike, Utc};
use std::fs::OpenOptions;
use std::io::Write;
use std::{error::Error, fs::create_dir_all};

use winsafe::{
    self as w, HKEY, RegistryValue,
    co::{self, KNOWNFOLDERID},
    prelude::*,
};

pub fn get_windows_path(folder_id: &KNOWNFOLDERID) -> Result<String, Box<dyn Error>> {
    let the_path = w::SHGetKnownFolderPath(folder_id, co::KF::DEFAULT, None)?;
    Ok(the_path)
}

pub fn set_dword(
    hkey: &HKEY,
    subkey: &str,
    value_name: &str,
    value: u32,
) -> Result<(), Box<dyn Error>> {
    let o_subkey = subkey;
    let hkey_text = get_hkey_text(hkey).unwrap();

    let (subkey, _) = hkey
        .RegCreateKeyEx(
            subkey,
            None,
            co::REG_OPTION::NON_VOLATILE,
            co::KEY::WRITE,
            None,
        )
        .expect(&format!(
            "Failed to open a DWORD in key: {} -> {} -> {}",
            hkey_text, o_subkey, value_name
        ));
    subkey
        .RegSetValueEx(Some(value_name), RegistryValue::Dword(value))
        .expect(&format!(
            "Failed to set a DWORD in key: {} -> {} -> {}",
            hkey_text, o_subkey, value_name
        ));

    log_registry(hkey, o_subkey, value_name, "DWORD").expect(&format!(
        "Failed to log a DWORD change for key: {} -> {} -> {}",
        hkey_text, o_subkey, value_name
    ));
    Ok(())
}

pub fn set_string(
    hkey: &HKEY,
    subkey: &str,
    value_name: &str,
    value: &str,
) -> Result<(), Box<dyn Error>> {
    let o_subkey = subkey;
    let hkey_text = get_hkey_text(hkey).unwrap();

    let (subkey, _) = hkey
        .RegCreateKeyEx(
            subkey,
            None,
            co::REG_OPTION::NON_VOLATILE,
            co::KEY::WRITE,
            None,
        )
        .expect(&format!(
            "Failed to open String key: {} -> {} -> {}",
            hkey_text, o_subkey, value_name
        ));
    let value = value.to_string();
    subkey
        .RegSetValueEx(Some(value_name), RegistryValue::Sz(value))
        .expect(&format!(
            "Failed to set String value in key: {} -> {} -> {}",
            hkey_text, o_subkey, value_name
        ));

    log_registry(hkey, o_subkey, value_name, "String").expect(&format!(
        "Failed to log String change for key: {} -> {} -> {}",
        hkey_text, o_subkey, value_name
    ));
    Ok(())
}

pub fn remove_subkey(hkey: &HKEY, subkey: &str) -> Result<(), Box<dyn Error>> {
    let o_subkey = subkey;
    let hkey_text = get_hkey_text(hkey).unwrap();

    match hkey.RegDeleteTree(Some(subkey)) {
        Ok(_) => Ok(()),
        Err(e) if e == w::co::ERROR::FILE_NOT_FOUND => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
    .expect(&format!(
        "Failed to delete subkey: {} -> {}",
        hkey_text, o_subkey
    ));

    log_registry(hkey, o_subkey, "->", "Removed")?;
    Ok(())
}

fn get_hkey_text(hkey: &HKEY) -> Result<&str, Box<dyn Error>> {
    let result = if *hkey == HKEY::LOCAL_MACHINE {
        "HKEY_LOCAL_MACHINE"
    } else if *hkey == HKEY::CURRENT_USER {
        "HKEY_CURRENT_USER"
    } else {
        "UNKNOWN_HKEY"
    };

    Ok(result)
}

fn log_registry(
    hkey: &HKEY,
    subkey: &str,
    value_name: &str,
    type_name: &str,
) -> Result<(), Box<dyn Error>> {
    let hkey_text = get_hkey_text(hkey).unwrap();

    let mut log_directory = get_windows_path(&KNOWNFOLDERID::Desktop)?;
    log_directory.push_str(r"\W11Boost Logs");

    create_dir_all(&log_directory)?;

    let now = Utc::now();
    let time_info = format!(
        "{}/{}/{} {}:{}:{}",
        now.day(),
        now.month(),
        now.year(),
        now.hour(),
        now.minute(),
        now.second()
    );

    let log_entry = format!(
        "{} -> {} -> {} -> {} -> {}\n",
        time_info, hkey_text, subkey, value_name, type_name
    );

    log_directory.push_str(r"\Registry.log");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_directory)?;

    file.write_all(log_entry.as_bytes())?;

    Ok(())
}
