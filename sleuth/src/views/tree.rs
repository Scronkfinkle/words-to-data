//! Tree navigator pane

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length, Padding};

use crate::message::Message;
use crate::state::AppState;
use crate::theme::colors;

impl AppState {
    /// Left panel: tree navigator (280px)
    pub fn view_tree_pane(&self) -> Element<Message> {
        // Load button
        let load_btn = button(text("Load...").size(11))
            .padding([4, 8])
            .on_press(Message::ToggleLoader);

        let header = row![
            text("Navigator").size(14).color(colors::TEXT_PRIMARY),
            load_btn
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        let tree_content: Element<Message> = if let Some(ref dataset) = self.dataset {
            if let Some(version) = dataset.versions.get(self.selected_version_index) {
                self.render_tree_node(&version.element, 0)
            } else {
                text("No version selected").into()
            }
        } else {
            text("No dataset loaded")
                .size(12)
                .color(colors::TEXT_SECONDARY)
                .into()
        };

        // Header with padding (top, right, left; no bottom)
        let header_container = container(header).padding(Padding::ZERO.top(8).right(8).left(8));

        // Tree content with left/bottom padding only (scrollbar on right edge)
        let tree_padded = container(tree_content).padding(Padding::ZERO.left(8).bottom(8));

        let content = column![
            header_container,
            scrollable(tree_padded)
                .height(Length::Fill)
                .width(Length::Fill)
        ]
        .spacing(8);

        container(content)
            .width(Length::Fixed(280.0))
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(colors::PAPER_DARK.into()),
                ..Default::default()
            })
            .into()
    }

    /// Render a tree node and its children recursively
    pub fn render_tree_node<'a>(
        &'a self,
        element: &'a words_to_data::uslm::USLMElement,
        depth: usize,
    ) -> Element<'a, Message> {
        let path = &element.data.path;
        let is_expanded = self.tree_expanded.contains(path);
        let is_selected = self.selected_path.as_ref() == Some(path);
        let has_children = !element.children.is_empty();

        // Check diff status
        let has_direct_change = self.changed_paths.contains(path);
        let has_child_changes = self.has_descendant_changes(path);

        // Build node row: [expand] [label] [badge]
        let mut node_row = row![].spacing(2).align_y(iced::Alignment::Center);

        // Expand/collapse button (only if has children)
        if has_children {
            let arrow = if is_expanded { "▼" } else { "▶" };
            let arrow_btn = button(text(arrow).size(10))
                .padding([2, 4])
                .style(|_, _| button::Style {
                    background: None,
                    text_color: colors::TEXT_SECONDARY,
                    ..Default::default()
                })
                .on_press(Message::TreeToggle(path.clone()));
            node_row = node_row.push(arrow_btn);
        } else {
            // Spacer for alignment
            node_row = node_row.push(container(text("")).width(18.0));
        }

        // Label (clickable to select)
        let label = if element.data.number_display.is_empty() {
            &element.data.verbose_name
        } else {
            &element.data.number_display
        };

        let label_color = if is_selected {
            colors::ACCENT
        } else {
            colors::TEXT_PRIMARY
        };

        let label_btn = button(text(label).size(13).color(label_color))
            .padding([2, 4])
            .style(move |_, status| {
                let bg = match status {
                    button::Status::Hovered => Some(colors::HOVER.into()),
                    _ if is_selected => Some(colors::SELECTION.into()),
                    _ => None,
                };
                button::Style {
                    background: bg,
                    text_color: label_color,
                    ..Default::default()
                }
            })
            .on_press(Message::SelectPath(path.clone()));
        node_row = node_row.push(label_btn);

        // Diff badge
        if has_direct_change {
            let badge = container(text("Δ").size(10).color(colors::TEXT_PRIMARY))
                .padding([1, 4])
                .style(|_| container::Style {
                    background: Some(colors::BADGE_CHANGED.into()),
                    border: iced::Border {
                        radius: 3.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            node_row = node_row.push(badge);
        } else if has_child_changes && !is_expanded {
            // Show dot indicator for collapsed nodes with child changes
            let dot = text("•").size(14).color(colors::BADGE_CHANGED);
            node_row = node_row.push(dot);
        }

        let mut col = column![node_row].spacing(1);

        // Children (if expanded)
        if has_children && is_expanded {
            for child in &element.children {
                let child_view = self.render_tree_node(child, depth + 1);
                let indented =
                    container(child_view).padding(iced::Padding::from([0.0, 0.0]).left(16.0));
                col = col.push(indented);
            }
        }

        col.into()
    }
}
