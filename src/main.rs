#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
use utils::request::make_request;

use std::error::Error;
use slint::Weak;
slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let ui_handle: Weak<AppWindow> = ui.as_weak();

    {
        let ui_weak = ui.as_weak();
        ui.on_fetch_data(move || {
            let ui = ui_weak.upgrade().expect("UI no longer exists");
            let url = ui.get_endpoint().to_string();
            let ui_handle = ui_handle.clone();
            tokio::spawn(async move {
                let response = make_request(url).await;
                let text = match response {
                    Ok(s) => slint::SharedString::from(s),
                    Err(e) => slint::SharedString::from(format!("Error: {}", e)),
                };
                if let Err(e) = ui_handle.upgrade_in_event_loop(move |ui| {
                    ui.set_response_text(text);
                }) {
                    eprintln!("Failed to upgrade UI in event loop: {:?}", e);
                }
            });
        });
    }

    ui.run()?;
    Ok(())
}
