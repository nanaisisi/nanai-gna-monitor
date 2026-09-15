use std::ffi::{CStr, CString};
use std::os::raw::c_void;

// Define a structure to hold the results from the GNA DLL.
// NOTE: The exact structure and types must match the actual C ABI of the DLL.
// This is a placeholder assuming the DLL exposes usage metrics like this.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GnomeUsageMetrics {
    pub total_gna_usage_bytes: u64,
    pub dedicated_gna_usage_bytes: u64,
    pub shared_gna_usage_bytes: u64,
}

// External function declaration to link against the DLL.
// This assumes a primary function that returns the usage data for all monitored processes/devices.
#[link(name = "nanai-gna-dll-load")]
extern "C" {
    fn get_gna_usage_metrics() -> GnomeUsageMetrics;
}

pub fn get_all_gna_usage() -> Result<GnomeUsageMetrics, String> {
    // Safety: We assume nanai-gna-dll-load exists and links correctly.
    // In a real scenario, we might check if the symbol is available or handle linking errors.
    let metrics = unsafe { get_gna_usage_metrics() };
    Ok(metrics)
}