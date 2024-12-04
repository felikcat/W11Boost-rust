use curl::easy::Easy;
use std::fs::File;
use std::path::Path;
use std::time::Duration;
use std::error::Error;
use std::io::Write;

pub fn install(mut path: String) -> Result<(), Box<dyn Error>> {
    let mut easy = Easy::new();

    easy.url("https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle")?;

    easy.useragent("Mozilla/5.0 (Windows NT 10.0; WOW64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6556.192 Safari/537.36")?;

    easy.follow_location(true)?;
    easy.connect_timeout(Duration::from_secs(10))?;

    path.push_str("\\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle");
    let mut file = File::create(Path::new(&path)).expect("appx_support::install -> Failed to create file");

    easy.write_function(move |data| {
        file.write_all(data).unwrap();
        Ok(data.len())
    }).expect("appx_support::install -> Failed to write data");

    easy.perform().expect("appx_support::install -> Failed to curl perform");

    std::process::Command::new(r#"(powershell.exe Add-AppxPackage ([Environment]::GetFolderPath("Desktop") + "\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle"))"#);

    Ok(())
}
