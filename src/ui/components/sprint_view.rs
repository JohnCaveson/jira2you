use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use crate::jira::Issue;

pub struct SprintView {
    pub issues: Vec<Issue>,
    pub state: ListState,
    pub sprint_name: String,
    pub sprint_goal: Option<String>,
}

impl SprintView {
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            state: ListState::default(),
            sprint_name: "Sprint".to_string(),
            sprint_goal: None,
        }
    }

    pub fn set_issues(&mut self, mut issues: Vec<Issue>, sprint_name: String, sprint_goal: Option<String>) {
        issues.sort_by(|a, b| b.key.cmp(&a.key));
        self.issues = issues;
        self.sprint_name = sprint_name;
        self.sprint_goal = sprint_goal;
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
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(area);

        // Sprint header
        let header_text = if let Some(goal) = &self.sprint_goal {
            format!("Sprint: {} - Goal: {}", self.sprint_name, goal)
        } else {
            format!("Sprint: {}", self.sprint_name)
        };
        let header = Paragraph::new(header_text)
            .block(Block::default().borders(Borders::ALL).title("Current Sprint"))
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(header, chunks[0]);

        // Issues list
        let items: Vec<ListItem> = self
            .issues
            .iter()
            .enumerate()
            .map(|(_i, issue)| {
                let status_color = match issue.fields.status.name.as_str() {
                    "To Do" | "Open" => Color::Red,
                    "In Progress" => Color::Yellow,
                    "Done" | "Closed" => Color::Green,
                    _ => Color::White,
                };

                let content = format!(
                    "{} [{}] {} - {}",
                    issue.key,
                    issue.fields.status.name,
                    issue.fields.summary,
                    issue.fields.assignee
                        .as_ref()
                        .map(|u| u.display_name.as_str())
                        .unwrap_or("Unassigned")
                );

                ListItem::new(content).style(Style::default().fg(status_color))
            })
            .collect();

        let issues_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Issues"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(issues_list, chunks[1], &mut self.state);
    }
}
