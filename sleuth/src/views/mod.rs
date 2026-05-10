//! View components for SLEUTH

mod detail;
mod modals;
mod reading;
mod timeline;
mod tree;

pub use detail::AnnotationDisplay;

use iced::widget::{column, container, row, stack};
use iced::{Element, Length, Subscription};

use crate::message::Message;
use crate::state::AppState;
use crate::theme::colors;

impl AppState {
    /// Keyboard subscription
    pub fn subscription(&self) -> Subscription<Message> {
        iced::keyboard::listen().map(|event| match event {
            iced::keyboard::Event::KeyPressed { key, modifiers, .. } => {
                Message::KeyPressed(key, modifiers)
            }
            // Ignore KeyReleased and ModifiersChanged
            _ => Message::NoOp,
        })
    }

    /// Render the view
    pub fn view(&self) -> Element<Message> {
        let content = row![
            self.view_tree_pane(),
            self.view_reading_pane(),
            self.view_changes_pane(),
        ]
        .spacing(1);

        let main = column![content, self.view_timeline()].spacing(1);

        let base: Element<Message> = container(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(colors::PAPER_BORDER.into()),
                ..Default::default()
            })
            .into();

        // Layer modals on top
        if self.show_search {
            stack![base, self.view_search_modal()].into()
        } else if self.show_loader {
            stack![base, self.view_loader_modal()].into()
        } else {
            base
        }
    }
}
