use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};
use crate::jira::Issue;

pub struct BacklogView {
    pub issues: Vec<Issue>,
    pub state: ListState,
}

impl BacklogView {
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn set_issues(&mut self, mut issues: Vec<Issue>) {
        issues.sort_by(|a, b| b.key.cmp(&a.key));
        self.issues = issues;
        if !self.issues.is_empty() {
            self.state.select(Some(0));
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.issues.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.issues.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected_issue(&self) -> Option<&Issue> {
        self.state.selected().and_then(|i| self.issues.get(i))
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .issues
            .iter()
            .map(|issue| {
                let priority_color = match issue.fields.priority.as_ref().map(|p| p.name.as_str()) {
                    Some("Highest") | Some("High") => Color::Red,
                    Some("Medium") => Color::Yellow,
                    Some("Low") | Some("Lowest") => Color::Green,
                    _ => Color::White,
                };

                let content = format!(
                    "{} [{}] {} - {}",
                    issue.key,
                    issue.fields.priority
                        .as_ref()
                        .map(|p| p.name.as_str())
                        .unwrap_or("None"),
                    issue.fields.summary,
                    issue.fields.assignee
                        .as_ref()
                        .map(|u| u.display_name.as_str())
                        .unwrap_or("Unassigned")
                );

                ListItem::new(content).style(Style::default().fg(priority_color))
            })
            .collect();

        let backlog_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Backlog"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(backlog_list, area, &mut self.state);
    }
}
