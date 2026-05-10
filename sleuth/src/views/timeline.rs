//! Timeline scrubber pane

use iced::widget::{button, column, container, row, slider, text};
use iced::{Element, Length};

use crate::message::Message;
use crate::state::AppState;
use crate::theme::colors;

impl AppState {
    /// Bottom panel: timeline with scrubber
    pub fn view_timeline(&self) -> Element<Message> {
        let Some(ref dataset) = self.dataset else {
            return container(text("")).height(Length::Fixed(56.0)).into();
        };

        let version_count = dataset.versions.len();
        if version_count == 0 {
            return container(text("No versions"))
                .height(Length::Fixed(56.0))
                .into();
        }

        let current_date = dataset
            .versions
            .get(self.selected_version_index)
            .map(|v| v.date.as_str())
            .unwrap_or("--");

        // Version label
        let version_label = dataset
            .versions
            .get(self.selected_version_index)
            .and_then(|v| v.label.as_deref())
            .unwrap_or("");

        let info = if version_label.is_empty() {
            text(format!(
                "{} ({}/{})",
                current_date,
                self.selected_version_index + 1,
                version_count
            ))
        } else {
            text(format!(
                "{} - {} ({}/{})",
                current_date,
                version_label,
                self.selected_version_index + 1,
                version_count
            ))
        }
        .size(12)
        .color(colors::TEXT_SECONDARY);

        // Navigation buttons
        let prev_btn = button(text("◀").size(12))
            .padding([4, 8])
            .on_press(Message::PrevVersion);

        let next_btn = button(text("▶").size(12))
            .padding([4, 8])
            .on_press(Message::NextVersion);

        // Slider (only if >1 version)
        let timeline_row: Element<Message> = if version_count > 1 {
            let max = (version_count - 1) as u32;
            let current = self.selected_version_index as u32;
            let scrubber = slider(0..=max, current, |v| Message::VersionChange(v as usize))
                .width(Length::Fill);

            row![prev_btn, scrubber, next_btn]
                .spacing(8)
                .align_y(iced::Alignment::Center)
                .into()
        } else {
            row![prev_btn, next_btn]
                .spacing(8)
                .align_y(iced::Alignment::Center)
                .into()
        };

        let content = column![info, timeline_row]
            .spacing(4)
            .align_x(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fixed(56.0))
            .padding(8)
            .center_x(Length::Fill)
            .style(|_| container::Style {
                background: Some(colors::PAPER_DARK.into()),
                ..Default::default()
            })
            .into()
    }
}
