use super::super::super::content_core::{group_info, group_metric_bytes};
use super::super::treemap_block;
use crate::category::ProcessGroup;
use crate::memory::{ProcessMemoryEntry, format_bytes};
use crate::treemap::TreemapNode;
use crate::ui::app_core::RammapApp;
use crate::ui::theme::color_for_category;
use crate::ui::types::RammapMessage;
use windows_reactor::*;

pub(super) fn build_group_node_view<S: Fn(RammapMessage) + Clone + 'static>(
    app: &RammapApp,
    sender: &S,
    index: usize,
    node: TreemapNode<ProcessGroup<ProcessMemoryEntry>>,
) -> View {
    let (rect, group) = node.into_parts();
    let selected = app.selected_group_title.as_deref() == Some(group.title());
    let background = if selected {
        Color::argb(255, 240, 160, 40)
    } else {
        color_for_category(group.category(), index as u32)
    };
    let border = if selected {
        Color::argb(255, 255, 255, 255)
    } else {
        Color::argb(200, 20, 20, 20)
    };
    let value = format_bytes(group_metric_bytes(&group, app.metric));
    let label = if rect.width() > 60.0 && rect.height() > 28.0 {
        if rect.height() > 44.0 && rect.width() > 80.0 {
            format!("{}\n{}", group.title(), value)
        } else {
            format!("{} ({})", group.title(), value)
        }
    } else if rect.width() > 35.0 && rect.height() > 18.0 {
        group.title().to_string()
    } else {
        String::new()
    };
    let first = group.items().first().cloned();
    let info = group_info(app.tab, &group);
    let click_sender = sender.clone();

    treemap_block(
        rect,
        background,
        border,
        3.0,
        label,
        if rect.height() > 38.0 { 11.5 } else { 9.5 },
        Thickness::xy(4.0, 3.0),
        move || {
            click_sender(RammapMessage::SelectProcess(
                first.clone(),
                Some(info.clone()),
            ));
        },
    )
}
