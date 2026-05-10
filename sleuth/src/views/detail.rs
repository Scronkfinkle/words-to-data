//! Detail/attribution pane (right side)

use iced::widget::{column, container, scrollable, text};
use iced::{Element, Length};

use words_to_data::annotation::{AnnotationStatus, ChangeAnnotation};
use words_to_data::uslm::AmendingAction;

use crate::message::Message;
use crate::state::AppState;
use crate::theme::colors;

/// Helper trait for annotation display
pub trait AnnotationDisplay {
    fn operation_display(&self) -> &'static str;
}

impl AnnotationDisplay for ChangeAnnotation {
    fn operation_display(&self) -> &'static str {
        match self.operation {
            AmendingAction::Amend => "amended",
            AmendingAction::Add => "added",
            AmendingAction::Delete => "deleted",
            AmendingAction::Insert => "inserted",
            AmendingAction::Redesignate => "redesignated",
            AmendingAction::Repeal => "repealed",
            AmendingAction::Move => "moved",
            AmendingAction::Strike => "struck",
            AmendingAction::StrikeAndInsert => "struck and inserted",
        }
    }
}

impl AppState {
    /// Right panel: detail pane (360px)
    /// Shows blame detail when selected, otherwise hint
    pub fn view_changes_pane(&self) -> Element<Message> {
        let content = if let Some(ref path) = self.blame_detail_path {
            self.render_blame_detail(path)
        } else {
            // Hint when nothing selected
            column![
                text("Attribution").size(16).color(colors::TEXT_PRIMARY),
                text("Click a bill label in the reading view to see annotation details")
                    .size(12)
                    .color(colors::TEXT_SECONDARY),
            ]
            .spacing(8)
            .into()
        };

        container(scrollable(content).height(Length::Fill))
            .width(Length::Fixed(360.0))
            .height(Length::Fill)
            .padding(12)
            .style(|_| container::Style {
                background: Some(colors::PAPER_DARK.into()),
                ..Default::default()
            })
            .into()
    }

    /// Render blame detail content for right pane
    pub fn render_blame_detail(&self, path: &str) -> Element<Message> {
        let Some(ref dataset) = self.dataset else {
            return text("No dataset").size(12).into();
        };

        let annotations = dataset.annotations_for_path(path);
        let Some(ann) = annotations.first() else {
            return text("No annotation for this path")
                .size(12)
                .color(colors::TEXT_SECONDARY)
                .into();
        };

        let mut content = column![].spacing(12);

        // Header
        let header = text(format!(
            "{} — {}",
            ann.source_bill.bill_id,
            ann.operation_display()
        ))
        .size(16)
        .color(colors::TEXT_PRIMARY);
        content = content.push(header);

        // Status
        let status_text = match ann.metadata.status {
            AnnotationStatus::Pending => "Pending review",
            AnnotationStatus::Verified => "Verified",
            AnnotationStatus::Disputed => "Disputed",
            AnnotationStatus::Rejected => "Rejected",
        };
        let status_color = match ann.metadata.status {
            AnnotationStatus::Verified => colors::INSERT_FG,
            AnnotationStatus::Rejected => colors::DELETE_FG,
            AnnotationStatus::Disputed => colors::BADGE_CHANGED,
            AnnotationStatus::Pending => colors::TEXT_SECONDARY,
        };
        content = content.push(text(status_text).size(11).color(status_color));

        // Causative text
        if !ann.source_bill.causative_text.is_empty() {
            let label = text("Causative Text")
                .size(11)
                .color(colors::TEXT_SECONDARY);
            let body = container(
                text(&ann.source_bill.causative_text)
                    .size(12)
                    .color(colors::TEXT_PRIMARY),
            )
            .padding(8)
            .style(|_| container::Style {
                background: Some(colors::PAPER.into()),
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            });
            content = content.push(column![label, body].spacing(4));
        }

        // Confidence
        if let Some(conf) = ann.metadata.confidence {
            content = content.push(
                text(format!("Confidence: {:.0}%", conf * 100.0))
                    .size(11)
                    .color(colors::TEXT_SECONDARY),
            );
        }

        // Annotator
        content = content.push(
            text(format!("Annotator: {}", ann.metadata.annotator))
                .size(11)
                .color(colors::TEXT_SECONDARY),
        );

        // Reasoning
        if let Some(ref reasoning) = ann.metadata.reasoning {
            let label = text("Reasoning").size(11).color(colors::TEXT_SECONDARY);
            let body = container(text(reasoning).size(12).color(colors::TEXT_PRIMARY))
                .padding(8)
                .style(|_| container::Style {
                    background: Some(colors::PAPER.into()),
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            content = content.push(column![label, body].spacing(4));
        }

        // Notes
        if let Some(ref notes) = ann.metadata.notes {
            let label = text("Notes").size(11).color(colors::TEXT_SECONDARY);
            let body = container(text(notes).size(12).color(colors::TEXT_PRIMARY))
                .padding(8)
                .style(|_| container::Style {
                    background: Some(colors::PAPER.into()),
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            content = content.push(column![label, body].spacing(4));
        }

        content.into()
    }
}
