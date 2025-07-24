use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use crate::jira::Board;

pub struct BoardSelector {
    pub boards: Vec<Board>,
    pub state: ListState,
    pub is_active: bool,
}

impl BoardSelector {
    pub fn new() -> Self {
        Self {
            boards: Vec::new(),
            state: ListState::default(),
            is_active: false,
        }
    }

    pub fn set_boards(&mut self, mut boards: Vec<Board>) {
        boards.sort_by(|a, b| b.name.cmp(&a.name));
        self.boards = boards;
        // Select the first board by default
        if !self.boards.is_empty() {
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
                if i >= self.boards.len() - 1 {
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
                    self.boards.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected_board(&self) -> Option<&Board> {
        self.state.selected().and_then(|i| self.boards.get(i))
    }

    pub fn selected_board_id(&self) -> Option<u32> {
        self.selected_board().map(|b| b.id)
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        if self.boards.is_empty() {
            let no_boards = Paragraph::new("No boards available")
                .block(Block::default().borders(Borders::ALL).title("Board Selector"))
                .style(Style::default().fg(Color::Gray));
            f.render_widget(no_boards, area);
            return;
        }

        let items: Vec<ListItem> = self
            .boards
            .iter()
            .enumerate()
            .map(|(_i, board)| {
                let board_type_color = match board.board_type.as_str() {
                    "scrum" => Color::Green,
                    "kanban" => Color::Blue,
                    "simple" => Color::Yellow,
                    _ => Color::White,
                };

                let type_symbol = match board.board_type.as_str() {
                    "scrum" => "🏃",
                    "kanban" => "📋",
                    "simple" => "📝",
                    _ => "📊",
                };

                let project_info = if let Some(location) = &board.location {
                    if let Some(project_key) = &location.project_key {
                        format!(" ({})", project_key)
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                let content = format!(
                    "{} {} [{}]{}",
                    type_symbol,
                    board.name,
                    board.board_type.to_uppercase(),
                    project_info
                );

                ListItem::new(content).style(Style::default().fg(board_type_color))
            })
            .collect();

        let title = if self.is_active {
            "Board Selector (ACTIVE)"
        } else {
            "Board Selector"
        };

        let border_style = if self.is_active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        let boards_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title).border_style(border_style))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(boards_list, area, &mut self.state);
    }
}
