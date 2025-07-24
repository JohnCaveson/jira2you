use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use crate::jira::Project;

pub struct ProjectSelector {
    pub projects: Vec<Project>,
    pub state: ListState,
    pub is_active: bool,
}

impl ProjectSelector {
    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
            state: ListState::default(),
            is_active: false,
        }
    }

    pub fn set_projects(&mut self, mut projects: Vec<Project>) {
        projects.sort_by(|a, b| b.name.cmp(&a.name));
        self.projects = projects;
        // Select the first project by default
        if !self.projects.is_empty() {
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
                if i >= self.projects.len() - 1 {
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
                    self.projects.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected_project(&self) -> Option<&Project> {
        self.state.selected().and_then(|i| self.projects.get(i))
    }

    pub fn selected_project_id(&self) -> Option<String> {
        self.selected_project().map(|p| p.id.clone())
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        if self.projects.is_empty() {
            let no_projects = Paragraph::new("No projects available")
                .block(Block::default().borders(Borders::ALL).title("Project Selector"))
                .style(Style::default().fg(Color::Gray));
            f.render_widget(no_projects, area);
            return;
        }

        let items: Vec<ListItem> = self
            .projects
            .iter()
            .enumerate()
            .map(|(_i, project)| {
                let project_type_color = match project.project_type_key.as_str() {
                    "software" => Color::Green,
                    "service_desk" => Color::Blue,
                    "business" => Color::Yellow,
                    _ => Color::White,
                };

                let type_symbol = match project.project_type_key.as_str() {
                    "software" => "💻",
                    "service_desk" => "🎧",
                    "business" => "📊",
                    _ => "📁",
                };

                let content = format!(
                    "{} {} [{}]",
                    type_symbol,
                    project.name,
                    project.key
                );

                ListItem::new(content).style(Style::default().fg(project_type_color))
            })
            .collect();

        let title = if self.is_active {
            "Project Selector (ACTIVE)"
        } else {
            "Project Selector"
        };

        let border_style = if self.is_active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        let projects_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title).border_style(border_style))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(projects_list, area, &mut self.state);
    }
}
