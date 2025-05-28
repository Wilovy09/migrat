#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use slint::Weak;

slint::include_modules!();


fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let _ui_handle: Weak<AppWindow> = ui.as_weak();

    ui.run()?;
    Ok(())
}
