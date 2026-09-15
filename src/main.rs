#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gna_api;
mod memory;
mod treemap;
mod ui;

use ui::app::RammapApp;
use ui::types::AppInput;
use windows_reactor::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // In a real implementation, we would initialize GNA monitoring here:
    let _gna_metrics = gna_api::get_all_gna_usage()?; 
    
    App::run_component::<RammapApp>(AppInput)?;
    Ok(())
}
