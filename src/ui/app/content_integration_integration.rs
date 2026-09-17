use super::content_integration::{grouped_list, individual_list};
use super::content_integration_core::{grouped_treemap, individual_treemap};
use crate::memory::ProcessMemoryEntry;
use crate::ui::app_core::GnaApp;
use crate::ui::types::{GroupMode, GnaMessage, ViewMode};
use windows_reactor::View;

pub(super) fn build<S>(app: &GnaApp, sender: S, filtered: Vec<ProcessMemoryEntry>) -> View
where
    S: Fn(GnaMessage) + Clone + 'static,
{
    match app.view_mode {
        ViewMode::Treemap => match app.group_mode {
            GroupMode::Individual => individual_treemap(app, sender, filtered),
            GroupMode::ByName | GroupMode::ByCategory => grouped_treemap(app, sender, filtered),
        },
        ViewMode::List => match app.group_mode {
            GroupMode::Individual => individual_list(app, sender, filtered),
            GroupMode::ByName | GroupMode::ByCategory => grouped_list(app, sender, filtered),
        },
    }
}
