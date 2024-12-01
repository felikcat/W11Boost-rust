use std::error::Error;

use winsafe::{self as w, co::{self, KNOWNFOLDERID}, prelude::*, HrResult};

pub fn get_windows_path(folder_id: &KNOWNFOLDERID) -> Result<String, Box<dyn Error>> {
  let the_path = w::SHGetKnownFolderPath(folder_id, co::KF::DEFAULT, None)?;
  Ok(the_path)
}