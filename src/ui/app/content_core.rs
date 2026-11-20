use crate::category::ProcessGroup;
use crate::memory::{ProcessMemoryEntry, format_bytes};
use crate::ui::types::{MemoryMetric, MemoryTab};

pub(super) fn metric_value(p: &ProcessMemoryEntry, _metric: MemoryMetric) -> f64 {
    p.gna_usage_bytes as f64
}

pub(super) fn group_metric_value(
    group: &ProcessGroup<ProcessMemoryEntry>,
    _metric: MemoryMetric,
) -> f64 {
    group.items.iter().map(|p| p.gna_usage_bytes).sum::<u64>() as f64
}

pub(super) fn metric_bytes(p: &ProcessMemoryEntry, metric: MemoryMetric) -> u64 {
    metric_value(p, metric) as u64
}

pub(super) fn group_metric_bytes(
    group: &ProcessGroup<ProcessMemoryEntry>,
    metric: MemoryMetric,
) -> u64 {
    group_metric_value(group, metric) as u64
}

pub(super) fn group_info(tab: MemoryTab, group: &ProcessGroup<ProcessMemoryEntry>) -> String {
    match tab {
        MemoryTab::GnaNpu => format!(
            "{} | Count: {} | Total GNA: {}",
            group.title,
            group.items.len(),
            format_bytes(group.items.iter().map(|p| p.gna_usage_bytes).sum()),
        ),
        MemoryTab::SystemRam => format!(
            "{} | Total WS: {} | Total Private: {} | Count: {}",
            group.title,
            format_bytes(group.total_working_set),
            format_bytes(group.total_private),
            group.items.len()
        ),
        MemoryTab::GpuVram => format!(
            "{} | Total Dedicated VRAM: {} | Total Shared: {} | Count: {}",
            group.title,
            format_bytes(group.total_gpu_dedicated),
            format_bytes(group.total_gpu_shared),
            group.items.len()
        ),
    }
}

pub(super) fn sort_processes(processes: &mut [ProcessMemoryEntry], metric: MemoryMetric) {
    processes.sort_by_key(|process| std::cmp::Reverse(metric_bytes(process, metric)));
}

pub(super) fn sort_groups(groups: &mut [ProcessGroup<ProcessMemoryEntry>], metric: MemoryMetric) {
    groups.sort_by_key(|group| std::cmp::Reverse(group_metric_bytes(group, metric)));
}
