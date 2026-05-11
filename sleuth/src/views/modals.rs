//! Modal overlay views (search, loader)

use iced::widget::{button, column, container, stack, text};
use iced::{Element, Length};

use crate::message::Message;
use crate::state::AppState;
use crate::theme::colors;

impl AppState {
    /// Search modal overlay
    pub fn view_search_modal(&self) -> Element<Message> {
        let backdrop = button(text(""))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_, _| button::Style {
                background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5).into()),
                ..Default::default()
            })
            .on_press(Message::CloseOverlays);

        let input = iced::widget::text_input("Search elements...", &self.search_query)
            .on_input(Message::SearchQueryChanged)
            .on_submit(Message::SearchSubmit)
            .padding(12)
            .size(16);

        let hint = text("Press Enter to search, Escape to close")
            .size(11)
            .color(colors::TEXT_SECONDARY);

        let modal = container(column![input, hint].spacing(8))
            .width(500.0)
            .padding(16)
            .style(|_| container::Style {
                background: Some(colors::PAPER.into()),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: colors::PAPER_BORDER,
                },
                shadow: iced::Shadow {
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
                ..Default::default()
            });

        let centered = container(modal)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .padding(iced::Padding::from([0.0, 0.0]).top(100.0));

        stack![backdrop, centered].into()
    }

    /// Loader modal overlay
    pub fn view_loader_modal(&self) -> Element<Message> {
        let backdrop = button(text(""))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_, _| button::Style {
                background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5).into()),
                ..Default::default()
            })
            .on_press(Message::CloseOverlays);

        let title = text("Load Dataset").size(18).color(colors::TEXT_PRIMARY);

        let browse_btn = button(text("Browse for JSON file...").size(14))
            .padding([12, 24])
            .on_press(Message::OpenFilePicker);

        let hint = text("Select a .json dataset file")
            .size(11)
            .color(colors::TEXT_SECONDARY);

        let modal = container(column![title, browse_btn, hint].spacing(12))
            .width(400.0)
            .padding(20)
            .style(|_| container::Style {
                background: Some(colors::PAPER.into()),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: colors::PAPER_BORDER,
                },
                shadow: iced::Shadow {
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
                ..Default::default()
            });

        let centered = container(modal)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .padding(iced::Padding::from([0.0, 0.0]).top(100.0));

        stack![backdrop, centered].into()
    }
}
