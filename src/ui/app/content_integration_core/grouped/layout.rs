use super::super::super::content_core::group_metric_value;
use super::super::bounds;
use crate::category::{ProcessGroup, group_by_category, group_by_name};
use crate::memory::ProcessMemoryEntry;
use crate::treemap::{TreemapItem, TreemapNode, layout_treemap};
use crate::ui::app_core::RammapApp;
use crate::ui::types::GroupMode;

pub(super) fn layout_grouped_nodes(
    app: &RammapApp,
    processes: &[ProcessMemoryEntry],
) -> Vec<TreemapNode<ProcessGroup<ProcessMemoryEntry>>> {
    let groups = if app.group_mode == GroupMode::ByName {
        group_by_name(processes)
    } else {
        group_by_category(processes)
    };
    let items = groups
        .into_iter()
        .map(|g| TreemapItem::new(group_metric_value(&g, app.metric), g))
        .collect::<Vec<_>>();

    layout_treemap(&items, bounds())
}
