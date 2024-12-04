mod gui;
pub mod common;
use fltk::dialog;
use gui::draw_gui;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match draw_gui() {
        Ok(_) => println!("draw_gui() exited successfully"),
        Err(e) => dialog::alert(0,0, &format!("W11Boost -> draw_gui() failed: {}", e)),
    }
    Ok(())
}
