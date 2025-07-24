use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub struct HelpView;

impl HelpView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(area);

        let title = Paragraph::new("Jira TUI - Keyboard Shortcuts")
            .block(Block::default().borders(Borders::ALL).title("Help"))
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(title, chunks[0]);

        let keybindings = vec![
            ("General", vec![
                ("q", "Quit application"),
                ("h", "Show/hide help"),
                ("Tab", "Switch between views"),
                ("Esc", "Go back/cancel"),
            ]),
            ("Navigation", vec![
                ("j/↓", "Move down"),
                ("k/↑", "Move up"),
                ("Enter", "Select/Open"),
            ]),
            ("Sprint/Backlog View", vec![
                ("r", "Refresh issues"),
                ("Enter", "View issue details"),
                ("s", "Switch to sprint view"),
                ("b", "Switch to backlog view"),
            ]),
            ("Issue Detail View", vec![
                ("e", "Edit issue"),
                ("c", "Add comment"),
                ("t", "Show transitions"),
                ("Enter", "Apply transition (when in transition mode)"),
            ]),
            ("Edit Mode", vec![
                ("Ctrl+s", "Save changes"),
                ("Esc", "Cancel editing"),
            ]),
        ];

        let mut items = Vec::new();
        for (category, bindings) in keybindings {
            items.push(ListItem::new(Line::from(Span::styled(
                category,
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ))));
            
            for (key, description) in bindings {
                items.push(ListItem::new(Line::from(vec![
                    Span::styled(format!("  {:<10}", key), Style::default().fg(Color::Green)),
                    Span::raw(description),
                ])));
            }
            items.push(ListItem::new(""));
        }

        let help_list = List::new(items)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(help_list, chunks[1]);
    }
}
