use std::mem::size_of;
use windows::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};

// --- Core Structures and Constants ---

/// Represents the overall GNA usage reported by the DLL.
#[derive(Debug, Clone)]
pub struct GnomeUsageMetrics {
    pub total_gna_usage_bytes: u64,
    // Placeholder fields to maintain API compatibility if the DLL changes
    pub dedicated_gna_usage_bytes: u64,
    pub shared_gna_usage_bytes: u64,
}

/// Summarizes overall system resources relevant to GNA monitoring only.
#[derive(Debug, Clone, Default)]
pub struct SystemMemorySummary {
    // Total monitored capacity (e.g., Max available GNA units)
    pub total_gna_capacity_bytes: u64,
}

/// Represents the GNA usage metrics for a specific process or system component.
#[derive(Debug, Clone, Default)]
pub struct ProcessMemoryEntry {
    pub pid: u32,
    pub name: String,
    pub gna_usage_bytes: u64, // The primary metric: GNA utilization
}

// --- Utility Functions ---

/// Converts raw byte count to a human-readable string format.
pub fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.1} MB", b / MB)
    } else if b >= KB {
        format!("{:.0} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}

// --- Collection Functions (Simplified for GNA focus) ---

/// Collects system-wide GNA summary metrics from the DLL.
pub fn collect_system_gna_summary() -> SystemMemorySummary {
    SystemMemorySummary::default()
}

/// Placeholder/stub function to map PIDs to ProcessMemoryEntry with GNA data.
/// This assumes the external API (or a helper tool) will provide individual process usage.
// In reality, this would loop through processes and call DLL functions per PID if necessary.
pub fn collect_process_memory(all_processes: &[(u32, String)]) -> Vec<ProcessMemoryEntry> {
    all_processes.iter().map(|&(pid, ref name)| {
        ProcessMemoryEntry {
            pid,
            name: name.clone(),
            gna_usage_bytes: 0, // Actual GNA data collection logic must be implemented here
        }
    }).collect()
}
