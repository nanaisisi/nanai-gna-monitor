#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod category;
mod gna_service;
mod gpu;
mod memory;
mod treemap;
mod ui;

use std::error::Error;
use ui::app::RammapApp;
use ui::types::AppInput;
use windows_reactor::App;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting nanai-gna-monitor (Intel GNA & System Hardware Monitor)...");

    let status = gna_service::detect_gna_status();
    println!("[INFO] GNA Status: {:?}", status);

    App::run_component::<RammapApp>(AppInput)?;
    Ok(())
}
