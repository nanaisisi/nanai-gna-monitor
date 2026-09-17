use crate::memory::ProcessMemoryEntry;

/// The core application input state.
#[derive(Clone, PartialEq, Default)]
pub struct AppInput;

// --- UI Structure Redefinition (GNA Only) ---

/// Defines the primary view context for GNA monitoring.
/// Since we are single-purpose, this might consolidate or become unnecessary, 
/// but retaining an enum structure is safer for large codebases refactoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    // We default to a detailed list view showing individual processes and system totals.
    DetailedList,
}

/// Defines how the process metrics are grouped/filtered in the UI list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupMode {
    Individual, // Show every single monitored entity (e.g., PID-1234)
    ByGnaUsage,  // Group by descending GNA usage amount
}

/// Defines the specific metric being displayed for a process or system summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gnametric {
    /// Primary GNA utilization metric (the main focus).
    PrimaryGnaUsage, 
}

// --- Messaging System Redefinition ---

/// Messages sent to the application core from various UI components.
#[derive(Clone)]
pub enum AppMessage {
    RefreshMetrics, // Trigger a re-read of GNA data from gna_api.rs
    SetViewMode(ViewMode),
    SetGroupMode(GroupMode),
    SelectProcess(Option<ProcessMemoryEntry>, Option<String>),
}
