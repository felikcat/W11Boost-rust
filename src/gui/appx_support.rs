use std::{io, time::Duration};
use curl::easy::{Easy, Handler, WriteError};

struct MyHandler;
impl Handler for MyHandler {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.write(data);
        Ok(data.len())
    }
}

pub fn install_appx_support() { 
    let mut easy = Easy::new();
    
    easy.url("https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle").unwrap();

    easy.useragent("Mozilla/5.0 (Windows NT 10.0; WOW64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6556.192 Safari/537.36").unwrap();

    easy.follow_location(true).unwrap();
    easy.connect_timeout(Duration::from_secs(10)).unwrap();

    let mut my_handler = MyHandler;
    easy.write_function(move |data| Ok(my_handler.write(data).unwrap()));


}