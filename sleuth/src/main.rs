//! SLEUTH - Structured Legal Examination Using Tracked History
//!
//! A native Rust GUI for exploring versioned US Code with bill attribution.

mod message;
mod state;
mod theme;
mod update;
mod views;

use iced::{Task, Theme, window};

use message::Message;
use state::AppState;

/// Application title
const TITLE: &str = "SLEUTH - Legal Code Diff Viewer";

fn main() -> iced::Result {
    iced::application(AppState::boot, AppState::update, AppState::view)
        .title(TITLE)
        .theme(theme)
        .subscription(AppState::subscription)
        .window(window::Settings {
            size: iced::Size::new(1400.0, 900.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            ..Default::default()
        })
        .run()
}

fn theme(_: &AppState) -> Theme {
    Theme::Light
}

impl AppState {
    /// Boot function for iced - called once at startup
    pub fn boot() -> (Self, Task<Message>) {
        let state = Self::new();

        // Auto-load sample dataset if it exists
        let sample_path = "/home/jesse/code/rust/words_to_data/titles_7_26.json";
        if std::path::Path::new(sample_path).exists() {
            let task = Task::done(Message::LoadDataset(sample_path.to_string()));
            return (state, task);
        }

        (state, Task::none())
    }
}
