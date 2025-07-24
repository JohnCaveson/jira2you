use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use crate::jira::Sprint;

pub struct SprintSelector {
    pub sprints: Vec<Sprint>,
    pub state: ListState,
    pub is_active: bool,
}

impl SprintSelector {
    pub fn new() -> Self {
        Self {
            sprints: Vec::new(),
            state: ListState::default(),
            is_active: false,
        }
    }

    pub fn set_sprints(&mut self, mut sprints: Vec<Sprint>) {
        sprints.sort_by(|a, b| b.id.cmp(&a.id));
        self.sprints = sprints;
        // Select the first (most recent) sprint by default
        if !self.sprints.is_empty() {
            self.state.select(Some(0));
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn next(&mut self) {
        if !self.is_active {
            return;
        }
        
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.sprints.len() - 1 {
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
        if !self.is_active {
            return;
        }
        
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.sprints.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected_sprint(&self) -> Option<&Sprint> {
        self.state.selected().and_then(|i| self.sprints.get(i))
    }

    pub fn selected_sprint_id(&self) -> Option<u32> {
        self.selected_sprint().map(|s| s.id)
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        if self.sprints.is_empty() {
            let no_sprints = Paragraph::new("No sprints available")
                .block(Block::default().borders(Borders::ALL).title("Sprint Selector"))
                .style(Style::default().fg(Color::Gray));
            f.render_widget(no_sprints, area);
            return;
        }

        let items: Vec<ListItem> = self
            .sprints
            .iter()
            .enumerate()
            .map(|(_i, sprint)| {
                let status_color = match sprint.state.as_str() {
                    "active" => Color::Green,
                    "closed" => Color::Gray,
                    "future" => Color::Blue,
                    _ => Color::White,
                };

                let status_symbol = match sprint.state.as_str() {
                    "active" => "●",
                    "closed" => "✓",
                    "future" => "○",
                    _ => "•",
                };

                let date_info = if let Some(complete) = &sprint.complete_date {
                    format!(" (Completed: {})", complete.format("%d/%b/%y"))
                } else if let (Some(start), Some(end)) = (&sprint.start_date, &sprint.end_date) {
                    format!(
                        " ({} - {})",
                        start.format("%d/%b"),
                        end.format("%d/%b")
                    )
                } else {
                    String::new()
                };

                let content = format!(
                    "{} {} [{}]{}",
                    status_symbol,
                    sprint.name,
                    sprint.state.to_uppercase(),
                    date_info
                );

                ListItem::new(content).style(Style::default().fg(status_color))
            })
            .collect();

        let title = if self.is_active {
            "Sprint Selector (ACTIVE)"
        } else {
            "Sprint Selector"
        };

        let border_style = if self.is_active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        let sprints_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title).border_style(border_style))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(sprints_list, area, &mut self.state);
    }
}
