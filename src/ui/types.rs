use crate::memory::ProcessMemoryEntry;

#[allow(dead_code)]
pub trait TypesInterface {
    fn tab(&self) -> MemoryTab;
    fn view_mode(&self) -> ViewMode;
    fn group_mode(&self) -> GroupMode;
    fn metric(&self) -> MemoryMetric;
}

#[derive(Clone, PartialEq, Default)]
pub struct AppInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryTab {
    GnaNpu,
    SystemRam,
    GpuVram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Treemap,
    List,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupMode {
    Individual,
    ByName,
    ByCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryMetric {
    WorkingSet,
    PrivateWs,
    GpuDedicated,
    GpuShared,
}

#[derive(Clone)]
pub enum RammapMessage {
    Refresh,
    RunGnaBenchmark,
    ToggleGnaLoadMode,
    SetTab(MemoryTab),
    SearchChanged(String),
    SetViewMode(ViewMode),
    SetGroupMode(GroupMode),
    ToggleMetric,
    SelectProcess(Option<ProcessMemoryEntry>, Option<String>),
}
