#[path = "app/content_core.rs"]
mod content_core;
#[path = "app/content_integration.rs"]
mod content_integration;
#[path = "app/content_integration_core.rs"]
mod content_integration_core;
#[path = "app/content_integration_integration.rs"]
mod content_integration_integration;

use crate::category::classify_process;
use crate::memory::{ProcessMemoryEntry, format_bytes};
use crate::ui::types::{AppInput, GroupMode, MemoryMetric, MemoryTab, RammapMessage, ViewMode};
use windows_reactor::*;

pub use crate::ui::app_core::RammapApp;

impl Component for RammapApp {
    type Input = AppInput;
    type Message = RammapMessage;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self::new(input)
    }
    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        self.update_message(message);
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let sender = context.sender();
        let send = {
            let sender = sender.clone();
            move |message| {
                let _ = sender.send(message);
            }
        };
        let content =
            content_integration_integration::build(self, send.clone(), self.filtered_processes());
        StackPanel::new()
            .spacing(10.0)
            .margin(Thickness::uniform(14.0))
            .children((self.header(send.clone()), self.search_panel(send), content))
            .into()
    }
}

impl RammapApp {
    fn filtered_processes(&self) -> Vec<ProcessMemoryEntry> {
        let query = self.search_query.to_lowercase();
        self.processes
            .iter()
            .filter(|p| {
                query.is_empty() || {
                    let category = classify_process(p.name());
                    p.name().to_lowercase().contains(&query)
                        || p.pid().to_string().contains(&query)
                        || category.label().to_lowercase().contains(&query)
                }
            })
            .cloned()
            .collect()
    }

    fn header<S: Fn(RammapMessage) + Clone + 'static>(&self, send: S) -> View {
        let title = TextBlock::new()
            .text("Nanai GNA Monitor (Intel GNA 3.0 NPU & Hardware Edition)")
            .font_size(22.0)
            .font_weight(FontWeight::BOLD);
        let gna_status = crate::gna_service::get_gna_status();
        let gna_badge = TextBlock::new()
            .text(format!("  [{}]", gna_status.description()))
            .font_size(14.0)
            .foreground(Color::argb(255, 70, 160, 245));
        let title_row = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children((title, gna_badge));
        let gna_actions = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children((
                self.tab_button("🧠 Intel GNA NPU", MemoryTab::GnaNpu, send.clone()),
                self.message_button(
                    "⚡ Run GNA Benchmark",
                    RammapMessage::RunGnaBenchmark,
                    send.clone(),
                ),
                self.message_button(
                    if gna_status.is_load_mode() {
                        "🔥 Load Mode: ON"
                    } else {
                        "🔥 Load Mode: OFF"
                    },
                    RammapMessage::ToggleGnaLoadMode,
                    send.clone(),
                ),
                self.message_button("🔄 Refresh", RammapMessage::Refresh, send.clone()),
            ));
        let view_actions = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children((
                self.mode_button("📊 2D Tree", ViewMode::Treemap, send.clone()),
                self.mode_button("📋 List", ViewMode::List, send.clone()),
                self.group_button("🗂️ By Name", GroupMode::ByName, send.clone()),
                self.group_button("🏷️ By Category", GroupMode::ByCategory, send.clone()),
                self.group_button("🔹 Individual", GroupMode::Individual, send.clone()),
                self.message_button(self.metric_label(), RammapMessage::ToggleMetric, send),
            ));
        StackPanel::new()
            .spacing(4.0)
            .children((
                title_row,
                TextBlock::new().text(self.subtitle()).font_size(12.0),
                gna_actions,
                view_actions,
            ))
            .into()
    }

    fn subtitle(&self) -> String {
        let gna = crate::gna_service::get_gna_status();
        format!(
            "Intel GNA 3.0 NPU ({}) | Detected: {} | Loaded: {} | HW Usage: {:.1}% | GNA Procs: {} (Ext: {}) | Inferences: {} ({:.2} ms, {:.1} IPS) | {}",
            gna.device_name(),
            gna.is_detected(),
            gna.is_loaded(),
            gna.hw_usage_percentage(),
            gna.active_process_count(),
            gna.has_external_processes(),
            gna.completed_inferences(),
            gna.avg_latency_ms(),
            gna.throughput_ips(),
            gna.last_test_status()
        )
    }

    fn metric_label(&self) -> &'static str {
        match self.metric {
            MemoryMetric::WorkingSet => "Metric: Working Set",
            MemoryMetric::PrivateWs => "Metric: Private (Commit)",
            _ => "Metric: Working Set",
        }
    }

    fn message_button<S: Fn(RammapMessage) + Clone + 'static>(
        &self,
        text: impl Into<String>,
        message: RammapMessage,
        send: S,
    ) -> View {
        let text = text.into();
        Button::new()
            .on_click(move || send(message.clone()))
            .content(TextBlock::new().text(text))
            .into()
    }

    fn tab_button<S: Fn(RammapMessage) + Clone + 'static>(
        &self,
        text: &'static str,
        tab: MemoryTab,
        send: S,
    ) -> View {
        self.message_button(text, RammapMessage::SetTab(tab), send)
    }
    fn mode_button<S: Fn(RammapMessage) + Clone + 'static>(
        &self,
        text: &'static str,
        mode: ViewMode,
        send: S,
    ) -> View {
        self.message_button(text, RammapMessage::SetViewMode(mode), send)
    }
    fn group_button<S: Fn(RammapMessage) + Clone + 'static>(
        &self,
        text: &'static str,
        mode: GroupMode,
        send: S,
    ) -> View {
        self.message_button(text, RammapMessage::SetGroupMode(mode), send)
    }

    fn search_panel<S: Fn(RammapMessage) + Clone + 'static>(&self, send: S) -> View {
        let selected = if let Some(ref title) = self.selected_group_title {
            format!("🎯 Selected Group: {}", title)
        } else if let Some(ref p) = self.selected_process {
            format!(
                "🎯 [{}] {} ({}) | GNA / NPU: {}",
                p.pid(),
                p.name(),
                classify_process(p.name()).label(),
                if p.gna_usage_bytes() > 0 {
                    format_bytes(p.gna_usage_bytes())
                } else {
                    "-".to_string()
                }
            )
        } else {
            "💡 Click any 2D tile or table row to inspect details.".to_string()
        };
        let search_send = send.clone();
        let search = TextBox::new()
            .placeholder_text("Filter PID, name or category...")
            .text(self.search_query.clone())
            .width(280.0)
            .on_text_changed(move |text: std::rc::Rc<str>| {
                search_send(RammapMessage::SearchChanged(text.to_string()))
            });
        StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(12.0)
            .children((
                search,
                TextBlock::new()
                    .text(selected)
                    .font_size(12.0)
                    .vertical_alignment(VerticalAlignment::Center),
            ))
            .into()
    }
}
