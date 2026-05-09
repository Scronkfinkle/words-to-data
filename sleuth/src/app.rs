//! Main application state and view logic for SLEUTH

use std::collections::HashSet;

use iced::widget::{button, column, container, rich_text, row, scrollable, span, text};
use iced::{Element, Length, Task};

use words_to_data::dataset::Dataset;
use words_to_data::diff::{TextChangeType, TreeDiff};

use crate::message::{ChangesTab, Message, TimelineStyle, ViewMode};
use crate::theme::colors;

/// Main application state
pub struct AppState {
    /// Loaded dataset (None until loaded)
    pub dataset: Option<Dataset>,

    /// Currently selected element path
    pub selected_path: Option<String>,

    /// Current version index (0-based)
    pub selected_version_index: usize,

    /// Cached diff between previous and current version
    pub current_diff: Option<TreeDiff>,

    /// Set of paths with changes in current diff (for fast lookup)
    pub changed_paths: HashSet<String>,

    /// Set of expanded tree paths
    pub tree_expanded: HashSet<String>,

    /// Current view mode
    pub view_mode: ViewMode,

    /// Show blame gutter
    pub show_blame: bool,

    /// Timeline display style
    pub timeline_style: TimelineStyle,

    /// Active tab in changes panel
    pub changes_tab: ChangesTab,

    /// Search overlay visible
    pub show_search: bool,

    /// Loader overlay visible
    pub show_loader: bool,

    /// Current search query
    pub search_query: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            dataset: None,
            selected_path: None,
            selected_version_index: 0,
            current_diff: None,
            changed_paths: HashSet::new(),
            tree_expanded: HashSet::new(),
            view_mode: ViewMode::default(),
            show_blame: true,
            timeline_style: TimelineStyle::default(),
            changes_tab: ChangesTab::default(),
            show_search: false,
            show_loader: false,
            search_query: String::new(),
        }
    }
}

impl AppState {
    /// Create new app state
    pub fn new() -> Self {
        Self::default()
    }

    /// Recompute diff between previous and current version
    fn recompute_diff(&mut self) {
        self.current_diff = None;
        self.changed_paths.clear();

        let Some(ref dataset) = self.dataset else {
            return;
        };

        if self.selected_version_index == 0 || dataset.versions.len() < 2 {
            return;
        }

        let from = &dataset.versions[self.selected_version_index - 1];
        let to = &dataset.versions[self.selected_version_index];

        let diff = TreeDiff::from_elements(&from.element, &to.element);
        self.collect_changed_paths(&diff);
        self.current_diff = Some(diff);
    }

    /// Recursively collect paths with changes into changed_paths set
    fn collect_changed_paths(&mut self, diff: &TreeDiff) {
        if !diff.changes.is_empty() || !diff.added.is_empty() || !diff.removed.is_empty() {
            self.changed_paths.insert(diff.root_path.clone());
        }
        for child in &diff.child_diffs {
            self.collect_changed_paths(child);
        }
    }

    /// Check if path or any descendant has changes
    fn has_descendant_changes(&self, path: &str) -> bool {
        self.changed_paths.iter().any(|p| p.starts_with(path))
    }

    /// Get diff for specific path from cached tree diff
    fn get_diff_for_path(&self, path: &str) -> Option<&TreeDiff> {
        self.current_diff.as_ref()?.find(path)
    }

    /// Update state in response to a message
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TreeToggle(path) => {
                if self.tree_expanded.contains(&path) {
                    self.tree_expanded.remove(&path);
                } else {
                    self.tree_expanded.insert(path);
                }
                Task::none()
            }
            Message::SelectPath(path) => {
                self.selected_path = Some(path);
                Task::none()
            }
            Message::VersionChange(idx) => {
                if let Some(ref dataset) = self.dataset {
                    if idx < dataset.versions.len() {
                        self.selected_version_index = idx;
                        self.recompute_diff();
                    }
                }
                Task::none()
            }
            Message::NextVersion => {
                if let Some(ref dataset) = self.dataset {
                    if self.selected_version_index + 1 < dataset.versions.len() {
                        self.selected_version_index += 1;
                        self.recompute_diff();
                    }
                }
                Task::none()
            }
            Message::PrevVersion => {
                if self.selected_version_index > 0 {
                    self.selected_version_index -= 1;
                    self.recompute_diff();
                }
                Task::none()
            }
            Message::ToggleViewMode => {
                self.view_mode = match self.view_mode {
                    ViewMode::Reading => ViewMode::Structural,
                    ViewMode::Structural => ViewMode::Reading,
                };
                Task::none()
            }
            Message::ToggleBlame => {
                self.show_blame = !self.show_blame;
                Task::none()
            }
            Message::SetTimelineStyle(style) => {
                self.timeline_style = style;
                Task::none()
            }
            Message::SetChangesTab(tab) => {
                self.changes_tab = tab;
                Task::none()
            }
            Message::ToggleSearch => {
                self.show_search = !self.show_search;
                if self.show_search {
                    self.show_loader = false;
                }
                Task::none()
            }
            Message::ToggleLoader => {
                self.show_loader = !self.show_loader;
                if self.show_loader {
                    self.show_search = false;
                }
                Task::none()
            }
            Message::CloseOverlays => {
                self.show_search = false;
                self.show_loader = false;
                Task::none()
            }
            Message::LoadDataset(path) => {
                self.show_loader = false;
                match Dataset::load(&path) {
                    Ok(dataset) => Task::done(Message::DatasetLoaded(Box::new(dataset))),
                    Err(e) => Task::done(Message::DatasetError(e.to_string())),
                }
            }
            Message::DatasetLoaded(dataset) => {
                // Auto-expand root and first level
                if let Some(version) = dataset.versions.first() {
                    let root_path = version.element.data.path.clone();
                    self.tree_expanded.insert(root_path);
                }
                self.dataset = Some(*dataset);
                self.selected_version_index = 0;
                self.selected_path = None;
                self.recompute_diff();
                Task::none()
            }
            Message::DatasetError(err) => {
                eprintln!("Dataset error: {}", err);
                Task::none()
            }
            Message::SearchQueryChanged(query) => {
                self.search_query = query;
                Task::none()
            }
            Message::SearchSubmit => {
                // TODO: Implement search
                Task::none()
            }
        }
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

        container(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(colors::PAPER_BORDER.into()),
                ..Default::default()
            })
            .into()
    }

    /// Left panel: tree navigator (280px)
    fn view_tree_pane(&self) -> Element<Message> {
        let content = if let Some(ref dataset) = self.dataset {
            if let Some(version) = dataset.versions.get(self.selected_version_index) {
                self.render_tree_node(&version.element, 0)
            } else {
                text("No version selected").into()
            }
        } else {
            text("No dataset loaded\n\nPress ⌘O to load").into()
        };

        container(scrollable(content).height(Length::Fill))
            .width(Length::Fixed(280.0))
            .height(Length::Fill)
            .padding(8)
            .style(|_| container::Style {
                background: Some(colors::PAPER_DARK.into()),
                ..Default::default()
            })
            .into()
    }

    /// Render a tree node and its children recursively
    fn render_tree_node<'a>(
        &'a self,
        element: &'a words_to_data::uslm::USLMElement,
        _depth: usize,
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
                let child_view = self.render_tree_node(child, _depth + 1);
                let indented =
                    container(child_view).padding(iced::Padding::from([0.0, 0.0]).left(16.0));
                col = col.push(indented);
            }
        }

        col.into()
    }

    /// Center panel: reading view (flexible)
    fn view_reading_pane(&self) -> Element<Message> {
        let content = if let Some(ref dataset) = self.dataset {
            if let Some(ref path) = self.selected_path {
                if let Some(version) = dataset.versions.get(self.selected_version_index) {
                    if let Some(element) = version.element.find(path) {
                        // Build reading view with optional blame gutter
                        if self.show_blame {
                            let blame = self.render_blame_gutter(path);
                            let content = self.render_element_content(element);
                            row![blame, content].spacing(8).into()
                        } else {
                            self.render_element_content(element)
                        }
                    } else {
                        text("Element not found").into()
                    }
                } else {
                    text("No version").into()
                }
            } else {
                text("Select an element from the tree").into()
            }
        } else {
            text("Load a dataset to begin").into()
        };

        container(scrollable(content).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(16)
            .style(|_| container::Style {
                background: Some(colors::PAPER.into()),
                ..Default::default()
            })
            .into()
    }

    /// Render blame gutter showing bill attribution
    fn render_blame_gutter(&self, path: &str) -> Element<Message> {
        let Some(ref dataset) = self.dataset else {
            return container(text("")).width(4.0).into();
        };

        // Get annotations for this path
        let annotations = dataset.annotations_for_path(path);

        if annotations.is_empty() {
            return container(text("")).width(4.0).into();
        }

        // Get sponsor party for first annotation
        use words_to_data::congress::Party;
        let color = if let Some(ann) = annotations.first() {
            if let Some(sponsor_info) = dataset.get_sponsor_info(&ann.source_bill.bill_id) {
                if let Some(member) = dataset.get_member(&sponsor_info.sponsor) {
                    match &member.party {
                        Party::Republican => colors::PARTY_R,
                        Party::Democrat => colors::PARTY_D,
                        Party::Independent | Party::Other(_) => colors::PARTY_I,
                    }
                } else {
                    colors::TEXT_SECONDARY
                }
            } else {
                colors::TEXT_SECONDARY
            }
        } else {
            colors::TEXT_SECONDARY
        };

        container(text(""))
            .width(4.0)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(color.into()),
                ..Default::default()
            })
            .into()
    }

    /// Render element content for reading view with diff highlighting
    fn render_element_content<'a>(
        &'a self,
        element: &'a words_to_data::uslm::USLMElement,
    ) -> Element<'a, Message> {
        use words_to_data::uslm::TextContentField;

        let path = &element.data.path;
        let diff = self.get_diff_for_path(path);
        let mut content = column![].spacing(8);

        // Helper to render field with or without diff
        let render_field = |field: TextContentField,
                            field_text: &'a str,
                            size: f32|
         -> Element<'a, Message> {
            if let Some(diff) = diff {
                // Find change for this field
                if let Some(change) = diff.changes.iter().find(|c| c.field_name == field) {
                    // Render with word-level highlighting
                    let spans: Vec<iced::widget::text::Span<'a, (), iced::Font>> = change
                        .changes
                        .iter()
                        .map(|tc| {
                            let s = span::<(), iced::Font>(&tc.value);
                            match tc.tag {
                                TextChangeType::Insert => s
                                    .color(colors::INSERT_FG)
                                    .background(colors::INSERT_BG),
                                TextChangeType::Delete => s
                                    .color(colors::DELETE_FG)
                                    .background(colors::DELETE_BG)
                                    .strikethrough(true),
                                TextChangeType::Equal => s.color(colors::TEXT_PRIMARY),
                            }
                        })
                        .collect();
                    return rich_text(spans).size(size).into();
                }
            }
            // No diff - plain text
            text(field_text).size(size).color(colors::TEXT_PRIMARY).into()
        };

        // Heading
        if let Some(ref heading) = element.data.heading {
            content = content.push(render_field(TextContentField::Heading, heading, 20.0));
        }

        // Chapeau
        if let Some(ref chapeau) = element.data.chapeau {
            content = content.push(render_field(TextContentField::Chapeau, chapeau, 14.0));
        }

        // Content
        if let Some(ref body) = element.data.content {
            content = content.push(render_field(TextContentField::Content, body, 14.0));
        }

        // Proviso
        if let Some(ref proviso) = element.data.proviso {
            content = content.push(render_field(TextContentField::Proviso, proviso, 13.0));
        }

        // Children
        for child in &element.children {
            content = content.push(self.render_element_content(child));
        }

        // Continuation
        if let Some(ref continuation) = element.data.continuation {
            content = content.push(render_field(TextContentField::Continuation, continuation, 14.0));
        }

        content.into()
    }

    /// Right panel: changes panel (360px)
    fn view_changes_pane(&self) -> Element<Message> {
        let header = text("Changes").size(16).color(colors::TEXT_PRIMARY);

        let tab_labels = ["This Version", "All Paths", "Lifetime"];
        let tabs = row(tab_labels.iter().enumerate().map(|(i, label)| {
            let tab = match i {
                0 => ChangesTab::ThisVersion,
                1 => ChangesTab::AllPaths,
                _ => ChangesTab::Lifetime,
            };
            let is_active = self.changes_tab == tab;
            iced::widget::button(text(*label).size(12))
                .padding([4, 8])
                .style(move |_, _| iced::widget::button::Style {
                    background: if is_active {
                        Some(colors::SELECTION.into())
                    } else {
                        None
                    },
                    text_color: colors::TEXT_PRIMARY,
                    ..Default::default()
                })
                .on_press(Message::SetChangesTab(tab))
                .into()
        }))
        .spacing(4);

        let changes_content = self.render_changes_content();

        let content = column![header, tabs, changes_content].spacing(8);

        container(scrollable(content).height(Length::Fill))
            .width(Length::Fixed(360.0))
            .height(Length::Fill)
            .padding(8)
            .style(|_| container::Style {
                background: Some(colors::PAPER_DARK.into()),
                ..Default::default()
            })
            .into()
    }

    /// Render changes panel content based on active tab
    fn render_changes_content(&self) -> Element<Message> {
        let Some(ref dataset) = self.dataset else {
            return text("No dataset").into();
        };

        match self.changes_tab {
            ChangesTab::ThisVersion => {
                if dataset.versions.len() < 2 || self.selected_version_index == 0 {
                    return text("No previous version to compare").into();
                }
                let from_date = &dataset.versions[self.selected_version_index - 1].date;
                let to_date = &dataset.versions[self.selected_version_index].date;

                if let Some(annotations) = dataset.get_annotations(from_date, to_date) {
                    let mut col = column![].spacing(4);
                    for ann in annotations {
                        let bill_text = format!(
                            "Bill {}: {}",
                            ann.source_bill.bill_id,
                            ann.operation_display()
                        );
                        col = col.push(text(bill_text).size(12));
                    }
                    col.into()
                } else {
                    text("No annotations for this version").into()
                }
            }
            ChangesTab::AllPaths => text("All changed paths (TODO)").into(),
            ChangesTab::Lifetime => {
                if let Some(ref path) = self.selected_path {
                    let annotations = dataset.annotations_for_path(path);
                    if annotations.is_empty() {
                        text("No changes to this element").into()
                    } else {
                        let mut col = column![].spacing(4);
                        for ann in annotations {
                            let bill_text = format!("Bill {}", ann.source_bill.bill_id);
                            col = col.push(text(bill_text).size(12));
                        }
                        col.into()
                    }
                } else {
                    text("Select an element to see its history").into()
                }
            }
        }
    }

    /// Bottom panel: timeline
    fn view_timeline(&self) -> Element<Message> {
        let Some(ref dataset) = self.dataset else {
            return container(text("")).height(Length::Fixed(48.0)).into();
        };

        let version_count = dataset.versions.len();
        let current_date = dataset
            .versions
            .get(self.selected_version_index)
            .map(|v| v.date.as_str())
            .unwrap_or("--");

        let info = text(format!(
            "Version {} of {} | {}",
            self.selected_version_index + 1,
            version_count,
            current_date
        ))
        .size(12)
        .color(colors::TEXT_SECONDARY);

        let prev_btn = iced::widget::button(text("◀").size(14))
            .padding([4, 8])
            .on_press(Message::PrevVersion);

        let next_btn = iced::widget::button(text("▶").size(14))
            .padding([4, 8])
            .on_press(Message::NextVersion);

        let controls = row![prev_btn, info, next_btn]
            .spacing(16)
            .align_y(iced::Alignment::Center);

        container(controls)
            .width(Length::Fill)
            .height(Length::Fixed(48.0))
            .padding(8)
            .center_x(Length::Fill)
            .style(|_| container::Style {
                background: Some(colors::PAPER_DARK.into()),
                ..Default::default()
            })
            .into()
    }
}

/// Helper trait for annotation display
trait AnnotationDisplay {
    fn operation_display(&self) -> &'static str;
}

impl AnnotationDisplay for words_to_data::annotation::ChangeAnnotation {
    fn operation_display(&self) -> &'static str {
        use words_to_data::uslm::AmendingAction;
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
