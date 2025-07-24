use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct InputView {
    pub input: String,
    pub title: String,
    pub cursor_position: usize,
}

impl InputView {
    pub fn new(title: String) -> Self {
        Self {
            input: String::new(),
            title,
            cursor_position: 0,
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn pop_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {

        // Add cursor indicator
        let display_text = if self.cursor_position < self.input.len() {
            format!("{}|{}", 
                &self.input[..self.cursor_position], 
                &self.input[self.cursor_position..]
            )
        } else {
            format!("{}|", self.input)
        };

        let input_widget = Paragraph::new(display_text)
            .block(Block::default().borders(Borders::ALL).title(self.title.as_str()))
            .style(Style::default().fg(Color::White));

        f.render_widget(input_widget, area);
    }
}
