use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use crate::jira::{Issue, Transition};

pub struct IssueDetailView {
    pub issue: Option<Issue>,
    pub transitions: Vec<Transition>,
    pub transition_state: ListState,
    pub show_transitions: bool,
}

impl IssueDetailView {
    pub fn new() -> Self {
        Self {
            issue: None,
            transitions: Vec::new(),
            transition_state: ListState::default(),
            show_transitions: false,
        }
    }

    pub fn set_issue(&mut self, issue: Issue) {
        self.issue = Some(issue);
    }

    pub fn set_transitions(&mut self, transitions: Vec<Transition>) {
        self.transitions = transitions;
        if !self.transitions.is_empty() {
            self.transition_state.select(Some(0));
        }
    }

    pub fn next_transition(&mut self) {
        let i = match self.transition_state.selected() {
            Some(i) => {
                if i >= self.transitions.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.transition_state.select(Some(i));
    }

    pub fn previous_transition(&mut self) {
        let i = match self.transition_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.transitions.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.transition_state.select(Some(i));
    }

    pub fn selected_transition(&self) -> Option<&Transition> {
        self.transition_state.selected().and_then(|i| self.transitions.get(i))
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        if let Some(ref issue) = self.issue {
            if self.show_transitions {
                self.render_transitions(f, area);
            } else {
                self.render_issue_details(f, area, issue);
            }
        } else {
            let no_issue = Paragraph::new("No issue selected")
                .block(Block::default().borders(Borders::ALL).title("Issue Details"));
            f.render_widget(no_issue, area);
        }
    }

    fn render_issue_details(&self, f: &mut Frame, area: Rect, issue: &Issue) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(10),
                Constraint::Length(8),
            ])
            .split(area);

        // Title
        let title = Paragraph::new(format!("{}: {}", issue.key, issue.fields.summary))
            .block(Block::default().borders(Borders::ALL).title("Issue"))
            .style(Style::default().fg(Color::Cyan))
            .wrap(Wrap { trim: true });
        f.render_widget(title, chunks[0]);

        // Metadata
        let metadata_lines = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&issue.fields.status.name),
            ]),
            Line::from(vec![
                Span::styled("Assignee: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(
                    issue.fields.assignee
                        .as_ref()
                        .map(|u| u.display_name.as_str())
                        .unwrap_or("Unassigned"),
                ),
            ]),
            Line::from(vec![
                Span::styled("Type: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&issue.fields.issuetype.name),
            ]),
        ];

        let metadata = Paragraph::new(metadata_lines)
            .block(Block::default().borders(Borders::ALL).title("Details"));
        f.render_widget(metadata, chunks[1]);

        // Description
        let default_description = "No description".to_string();
        let description_text = issue.fields.description
            .as_ref()
            .unwrap_or(&default_description);
        
        let description = Paragraph::new(description_text.as_str())
            .block(Block::default().borders(Borders::ALL).title("Description"))
            .wrap(Wrap { trim: true });
        f.render_widget(description, chunks[2]);

        // Comments
        if let Some(ref comments) = issue.fields.comment {
            let comment_items: Vec<ListItem> = comments
                .comments
                .iter()
                .map(|comment| {
                    let content = format!(
                        "{}: {}",
                        comment.author.display_name,
                        comment.body
                    );
                    ListItem::new(content)
                })
                .collect();

            let comments_list = List::new(comment_items)
                .block(Block::default().borders(Borders::ALL).title("Comments"));
            f.render_widget(comments_list, chunks[3]);
        } else {
            let no_comments = Paragraph::new("No comments")
                .block(Block::default().borders(Borders::ALL).title("Comments"));
            f.render_widget(no_comments, chunks[3]);
        }
    }

    fn render_transitions(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .transitions
            .iter()
            .map(|transition| {
                ListItem::new(format!("{} -> {}", transition.name, transition.to.name))
            })
            .collect();

        let transitions_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Available Transitions"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(transitions_list, area, &mut self.transition_state);
    }
}
