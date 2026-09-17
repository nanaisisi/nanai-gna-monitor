use std::ffi::{CStr, CString};
use std::os::raw::c_void;

// =============================================================================
// GNA DLL API Wrapper
// This module exclusively handles interaction with the external nanai-gna-dll.
// All logic is scoped to reading GNA usage metrics.
// =============================================================================

/// Structure matching the GNA data provided by the native DLL.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GnomeUsageMetrics {
    /// Total combined GNA utilization across all monitored scopes (system-wide).
    pub total_gna_usage_bytes: u64,
    /// Dedicated GNA usage metrics (specific to a core function/module).
    pub dedicated_gna_usage_bytes: u64,
    /// Shared GNA usage metrics (shared pool utilization).
    pub shared_gna_usage_bytes: u64,
}

// External function declaration to link against the DLL.
#[link(name = "nanai-gna-dll-load")]
extern "C" {
    fn get_gna_usage_metrics() -> GnomeUsageMetrics;
}

/// Attempts to retrieve all GNA usage metrics from the loaded native library.
pub fn get_all_gna_usage() -> Result<GnomeUsageMetrics, String> {
    // Safety: We assume nanai-gna-dll-load exists and links correctly at runtime.
    let metrics = unsafe { get_gna_usage_metrics() };
    Ok(metrics)
}