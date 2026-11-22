#[path = "grouped/layout.rs"]
mod layout;
#[path = "grouped/node.rs"]
mod node;

use super::wrap_treemap_canvas;
use crate::memory::ProcessMemoryEntry;
use crate::ui::app_core::RammapApp;
use crate::ui::types::RammapMessage;
use windows_reactor::View;

pub(in super::super) fn grouped_treemap<S: Fn(RammapMessage) + Clone + 'static>(
    app: &RammapApp,
    sender: S,
    processes: Vec<ProcessMemoryEntry>,
) -> View {
    let elements = layout::layout_grouped_nodes(app, &processes)
        .into_iter()
        .enumerate()
        .map(|(index, node)| node::build_group_node_view(app, &sender, index, node))
        .collect();

    wrap_treemap_canvas(elements)
}
