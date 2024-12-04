use std::error::Error;

use winsafe::{self as w, co::{self, KNOWNFOLDERID}, prelude::*, HrResult, RegistryValue, HKEY};

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
  let (subkey, _) = hkey.RegCreateKeyEx(
      subkey,
      None,
      co::REG_OPTION::NON_VOLATILE,
      co::KEY::WRITE,
      None,
  )?;
  subkey.RegSetValueEx(Some(value_name), RegistryValue::Dword(value))?;
  Ok(())
}

pub fn remove_subkey(hkey: &HKEY, subkey: &str) -> Result<(), Box<dyn Error>> {
  hkey.RegDeleteTree(Some(subkey))?;
  Ok(())
}