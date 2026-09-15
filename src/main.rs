#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gna_api;
mod memory;
mod treemap;
mod ui;

use ui::app::RammapApp;
use ui::types::AppInput;
use windows_reactor::App;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let gna_metrics = match gna_api::get_all_gna_usage() {
        Ok(metrics) => {
            println!("[INFO] Successfully initialized GNA monitoring. Total Usage: {}", format_bytes(metrics.total_gna_usage_bytes));
            Some(metrics)
        },
        Err(e) => {
            // Handle the DLL failure gracefully and report it to the user/log.
            eprintln!("[ERROR] Failed to initialize GNA monitoring API: {}. The application will run in placeholder mode.", e);
            None
        }
    };

    App::run_component::<RammapApp>(AppInput)?;
    Ok(())
}
