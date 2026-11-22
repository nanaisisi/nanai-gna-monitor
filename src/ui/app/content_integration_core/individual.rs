use super::super::content_core::{metric_bytes, metric_value};
use super::{bounds, treemap_block, wrap_treemap_canvas};
use crate::category::classify_process;
use crate::memory::{ProcessMemoryEntry, format_bytes};
use crate::treemap::{TreemapItem, layout_treemap};
use crate::ui::app_core::RammapApp;
use crate::ui::theme::color_for_category;
use crate::ui::types::RammapMessage;
use windows_reactor::*;

pub(in super::super) fn individual_treemap<S: Fn(RammapMessage) + Clone + 'static>(
    app: &RammapApp,
    sender: S,
    processes: Vec<ProcessMemoryEntry>,
) -> View {
    let items = processes
        .into_iter()
        .map(|p| TreemapItem::new(metric_value(&p, app.metric), p))
        .collect::<Vec<_>>();
    let mut elements = Vec::new();

    for node in layout_treemap(&items, bounds()) {
        let (rect, p) = node.into_parts();
        let pid = p.pid();
        let label_value = format_bytes(metric_bytes(&p, app.metric));
        let category = classify_process(p.name());
        let selected = app
            .selected_process
            .as_ref()
            .is_some_and(|s| s.pid() == pid);
        let background = if selected {
            Color::argb(255, 240, 160, 40)
        } else {
            color_for_category(category, pid)
        };
        let border = if selected {
            Color::argb(255, 255, 255, 255)
        } else {
            Color::argb(180, 20, 20, 20)
        };
        let label = if rect.width() > 55.0 && rect.height() > 26.0 {
            if rect.height() > 42.0 && rect.width() > 70.0 {
                format!("{}\n{}", p.name(), label_value)
            } else {
                format!("{} ({})", p.name(), label_value)
            }
        } else if rect.width() > 30.0 && rect.height() > 18.0 {
            p.name().to_string()
        } else {
            String::new()
        };
        let selected_process = p.clone();
        let click_sender = sender.clone();
        let block = treemap_block(
            rect,
            background,
            border,
            2.0,
            label,
            if rect.height() > 35.0 { 11.0 } else { 9.5 },
            Thickness::xy(3.0, 2.0),
            move || {
                click_sender(RammapMessage::SelectProcess(
                    Some(selected_process.clone()),
                    None,
                ));
            },
        );
        elements.push(block);
    }
    wrap_treemap_canvas(elements)
}
