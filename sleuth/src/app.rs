//! Main application state and view logic for SLEUTH

use std::collections::HashSet;

use iced::keyboard::{Key, Modifiers, key::Named};
use iced::widget::{
    button, column, container, rich_text, row, scrollable, slider, span, stack, text, text_input,
};
use iced::{Element, Length, Subscription, Task};

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

    /// Loader path input
    pub loader_path: String,
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
            loader_path: String::new(),
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
            Message::LoaderPathChanged(path) => {
                self.loader_path = path;
                Task::none()
            }
            Message::KeyPressed(key, modifiers) => {
                // Handle keyboard shortcuts
                if modifiers.command() {
                    match key.as_ref() {
                        Key::Character("k") => {
                            self.show_search = !self.show_search;
                            self.show_loader = false;
                        }
                        Key::Character("o") => {
                            self.show_loader = !self.show_loader;
                            self.show_search = false;
                        }
                        _ => {}
                    }
                }
                // Escape closes overlays
                if key == Key::Named(Named::Escape) {
                    self.show_search = false;
                    self.show_loader = false;
                }
                Task::none()
            }
        }
    }

    /// Keyboard subscription
    pub fn subscription(&self) -> Subscription<Message> {
        iced::keyboard::listen().map(|event| {
            if let iced::keyboard::Event::KeyPressed { key, modifiers, .. } = event {
                Message::KeyPressed(key, modifiers)
            } else {
                // Ignore other keyboard events
                Message::CloseOverlays // Dummy, won't trigger
            }
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

    /// Search modal overlay
    fn view_search_modal(&self) -> Element<Message> {
        let backdrop = button(text(""))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_, _| button::Style {
                background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5).into()),
                ..Default::default()
            })
            .on_press(Message::CloseOverlays);

        let input = text_input("Search elements...", &self.search_query)
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
    fn view_loader_modal(&self) -> Element<Message> {
        let backdrop = button(text(""))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_, _| button::Style {
                background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5).into()),
                ..Default::default()
            })
            .on_press(Message::CloseOverlays);

        let title = text("Load Dataset").size(18).color(colors::TEXT_PRIMARY);

        let input = text_input("Path to dataset JSON...", &self.loader_path)
            .on_input(Message::LoaderPathChanged)
            .on_submit(Message::LoadDataset(self.loader_path.clone()))
            .padding(12)
            .size(14);

        let load_btn = button(text("Load").size(14))
            .padding([8, 16])
            .on_press(Message::LoadDataset(self.loader_path.clone()));

        let hint = text("Enter path to .json dataset file")
            .size(11)
            .color(colors::TEXT_SECONDARY);

        let modal = container(column![title, input, load_btn, hint].spacing(12))
            .width(500.0)
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
                        self.render_element_content(element)
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

    /// Get bill attribution info for a path (bill_id, party color)
    fn get_blame_for_path(&self, path: &str) -> Option<(String, iced::Color)> {
        use words_to_data::congress::Party;

        let dataset = self.dataset.as_ref()?;
        let annotations = dataset.annotations_for_path(path);
        let ann = annotations.first()?;

        let bill_id = &ann.source_bill.bill_id;

        // Format as "hr7024·118" style
        let formatted_id = bill_id.replace("-", "·");

        let color = if let Some(sponsor_info) = dataset.get_sponsor_info(bill_id) {
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
        };

        Some((formatted_id, color))
    }

    /// Render element content for reading view with diff highlighting and blame
    fn render_element_content<'a>(
        &'a self,
        element: &'a words_to_data::uslm::USLMElement,
    ) -> Element<'a, Message> {
        use words_to_data::uslm::TextContentField;

        let path = &element.data.path;
        let diff = self.get_diff_for_path(path);
        let has_changes = diff.is_some_and(|d| !d.changes.is_empty());

        let mut fields_content = column![].spacing(4);

        // Helper to render a field
        let render_field =
            |field: TextContentField, field_text: &'a str, size: f32| -> Element<'a, Message> {
                if let Some(diff) = diff {
                    if let Some(change) = diff.changes.iter().find(|c| c.field_name == field) {
                        let spans: Vec<iced::widget::text::Span<'a, (), iced::Font>> = change
                            .changes
                            .iter()
                            .map(|tc| {
                                let s = span::<(), iced::Font>(&tc.value);
                                match tc.tag {
                                    TextChangeType::Insert => {
                                        s.color(colors::INSERT_FG).background(colors::INSERT_BG)
                                    }
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
                text(field_text)
                    .size(size)
                    .color(colors::TEXT_PRIMARY)
                    .into()
            };

        // Heading
        if let Some(ref heading) = element.data.heading {
            fields_content =
                fields_content.push(render_field(TextContentField::Heading, heading, 20.0));
        }

        // Chapeau
        if let Some(ref chapeau) = element.data.chapeau {
            fields_content =
                fields_content.push(render_field(TextContentField::Chapeau, chapeau, 14.0));
        }

        // Content
        if let Some(ref body) = element.data.content {
            fields_content =
                fields_content.push(render_field(TextContentField::Content, body, 14.0));
        }

        // Proviso
        if let Some(ref proviso) = element.data.proviso {
            fields_content =
                fields_content.push(render_field(TextContentField::Proviso, proviso, 13.0));
        }

        // Wrap with blame gutter if has changes
        let element_row: Element<'a, Message> = if has_changes && self.show_blame {
            if let Some((bill_id, color)) = self.get_blame_for_path(path) {
                // Party color stripe
                let stripe =
                    container(text(""))
                        .width(4.0)
                        .height(Length::Shrink)
                        .style(move |_| container::Style {
                            background: Some(color.into()),
                            ..Default::default()
                        });

                // Bill label
                let label = text(bill_id).size(10).color(colors::TEXT_SECONDARY);

                let blame_col = column![stripe, label].spacing(2);
                row![blame_col, fields_content].spacing(8).into()
            } else {
                fields_content.into()
            }
        } else {
            fields_content.into()
        };

        // Build final with children
        let mut final_content = column![element_row].spacing(8);

        // Children
        for child in &element.children {
            final_content = final_content.push(self.render_element_content(child));
        }

        // Continuation (after children, no blame wrap)
        if let Some(ref continuation) = element.data.continuation {
            final_content = final_content.push(render_field(
                TextContentField::Continuation,
                continuation,
                14.0,
            ));
        }

        final_content.into()
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
            ChangesTab::AllPaths => {
                if self.changed_paths.is_empty() {
                    return text("No changes in this version").into();
                }

                let mut col = column![].spacing(2);
                let mut sorted_paths: Vec<_> = self.changed_paths.iter().collect();
                sorted_paths.sort();

                for path in sorted_paths {
                    // Extract short name from path
                    let short_name: String =
                        path.rsplit('/').next().unwrap_or(path).replace('_', " ");

                    let is_selected = self.selected_path.as_ref() == Some(path);
                    let path_clone = path.clone();
                    let path_btn = button(text(short_name).size(11))
                        .padding([2, 4])
                        .style(move |_, status| {
                            let bg = match status {
                                button::Status::Hovered => Some(colors::HOVER.into()),
                                _ if is_selected => Some(colors::SELECTION.into()),
                                _ => None,
                            };
                            button::Style {
                                background: bg,
                                text_color: colors::TEXT_PRIMARY,
                                ..Default::default()
                            }
                        })
                        .on_press(Message::SelectPath(path_clone));
                    col = col.push(path_btn);
                }
                col.into()
            }
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

    /// Bottom panel: timeline with scrubber
    fn view_timeline(&self) -> Element<Message> {
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
