use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::config::Config;
use crate::jira::JiraClient;
use crate::ui::components::{BacklogView, HelpView, InputView, IssueDetailView, SprintView, SprintSelector, BoardSelector, ProjectSelector};
use crate::ui::events::Event;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Sprint,
    SprintSelector,
    BoardSelector,
    ProjectSelector,
    Backlog,
    IssueDetail,
    Help,
    AddComment,
    EditIssue,
    EditSprintName,
}

pub struct App {
    pub mode: AppMode,
    pub show_help: bool,
    pub jira_client: JiraClient,
    pub config: Config,
    
    // Views
    pub sprint_view: SprintView,
    pub sprint_selector: SprintSelector,
    pub board_selector: BoardSelector,
    pub project_selector: ProjectSelector,
    pub backlog_view: BacklogView,
    pub issue_detail_view: IssueDetailView,
    pub help_view: HelpView,
    pub input_view: InputView,
    
    // State
    pub current_tab: usize,
    pub should_quit: bool,
    pub current_sprint_id: Option<u32>,
    pub available_boards: Vec<crate::jira::Board>,
    pub available_sprints: Vec<crate::jira::Sprint>,
    pub available_projects: Vec<crate::jira::Project>,
}

impl App {
    pub fn new(config: Config) -> Self {
        let jira_client = JiraClient::new(
            config.jira.username.clone(),
            config.jira.api_token.clone(),
            config.jira.domain.clone(),
        );

        Self {
            mode: AppMode::Sprint,
            show_help: false,
            jira_client,
            config,
            sprint_view: SprintView::new(),
            sprint_selector: SprintSelector::new(),
            board_selector: BoardSelector::new(),
            project_selector: ProjectSelector::new(),
            backlog_view: BacklogView::new(),
            issue_detail_view: IssueDetailView::new(),
            help_view: HelpView::new(),
            input_view: InputView::new("Input".to_string()),
            current_tab: 0,
            should_quit: false,
            current_sprint_id: None,
            available_boards: Vec::new(),
            available_sprints: Vec::new(),
            available_projects: Vec::new(),
        }
    }

    pub async fn handle_event(&mut self, event: Event) -> Result<bool> {
        match event {
            Event::Key(key, modifiers) => {
                if self.show_help {
                    return self.handle_help_input(key).await;
                }

                match self.mode {
                    AppMode::Sprint => self.handle_sprint_input(key, modifiers).await?,
                    AppMode::SprintSelector => self.handle_sprint_selector_input(key, modifiers).await?,
                    AppMode::BoardSelector => self.handle_board_selector_input(key, modifiers).await?,
                    AppMode::ProjectSelector => self.handle_project_selector_input(key, modifiers).await?,
                    AppMode::Backlog => self.handle_backlog_input(key, modifiers).await?,
                    AppMode::IssueDetail => self.handle_issue_detail_input(key, modifiers).await?,
                    AppMode::AddComment => self.handle_comment_input(key, modifiers).await?,
                    AppMode::EditIssue => self.handle_edit_input(key, modifiers).await?,
                    AppMode::EditSprintName => self.handle_edit_sprint_name_input(key, modifiers).await?,
                    AppMode::Help => { self.handle_help_input(key).await?; }
                }
            }
            Event::Tick => {
                // Handle periodic updates
            }
            Event::Quit => {
                self.should_quit = true;
            }
        }

        Ok(self.should_quit)
    }

    async fn handle_sprint_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> Result<()> {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('h') => self.show_help = !self.show_help,
            KeyCode::Char('s') => self.mode = AppMode::Sprint,
            KeyCode::Char('b') => {
                self.mode = AppMode::Backlog;
                self.load_backlog().await?;
            }
            KeyCode::Char('r') => self.refresh_sprint().await?,
            KeyCode::Tab => {
                // Switch to sprint selector
                self.sprint_selector.set_sprints(self.available_sprints.clone());
                self.sprint_selector.activate();
                self.mode = AppMode::SprintSelector;
            }
            KeyCode::Char('B') => {
                // Switch to board selector (capital B for board selector)
                self.board_selector.set_boards(self.available_boards.clone());
                self.board_selector.activate();
                self.mode = AppMode::BoardSelector;
            }
            KeyCode::Char('P') => {
                // Switch to project selector (capital P for project selector)
                self.project_selector.set_projects(self.available_projects.clone());
                self.project_selector.activate();
                self.mode = AppMode::ProjectSelector;
            }
            KeyCode::Down | KeyCode::Char('j') => self.sprint_view.next(),
            KeyCode::Up | KeyCode::Char('k') => self.sprint_view.previous(),
            KeyCode::Enter => {
                if let Some(issue) = self.sprint_view.selected_issue() {
                    let issue_key = issue.key.clone();
                    self.issue_detail_view.set_issue(issue.clone());
                    self.load_transitions(&issue_key).await?;
                    self.mode = AppMode::IssueDetail;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_backlog_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> Result<()> {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('h') => self.show_help = !self.show_help,
            KeyCode::Char('s') => {
                self.mode = AppMode::Sprint;
                self.refresh_sprint().await?;
            }
            KeyCode::Char('b') => self.mode = AppMode::Backlog,
            KeyCode::Char('r') => self.load_backlog().await?,
            KeyCode::Down | KeyCode::Char('j') => self.backlog_view.next(),
            KeyCode::Up | KeyCode::Char('k') => self.backlog_view.previous(),
            KeyCode::Enter => {
                if let Some(issue) = self.backlog_view.selected_issue() {
                    let issue_key = issue.key.clone();
                    self.issue_detail_view.set_issue(issue.clone());
                    self.load_transitions(&issue_key).await?;
                    self.mode = AppMode::IssueDetail;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_issue_detail_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> Result<()> {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('h') => self.show_help = !self.show_help,
            KeyCode::Esc => {
                if self.issue_detail_view.show_transitions {
                    self.issue_detail_view.show_transitions = false;
                } else {
                    self.mode = AppMode::Sprint;
                }
            }
            KeyCode::Char('c') => {
                self.input_view = InputView::new("Add Comment".to_string());
                self.mode = AppMode::AddComment;
            }
            KeyCode::Char('e') => {
                self.input_view = InputView::new("Edit Issue Summary".to_string());
                if let Some(issue) = &self.issue_detail_view.issue {
                    self.input_view.input = issue.fields.summary.clone();
                    self.input_view.cursor_position = self.input_view.input.len();
                }
                self.mode = AppMode::EditIssue;
            }
            KeyCode::Char('t') => {
                self.issue_detail_view.show_transitions = true;
            }
            KeyCode::Down | KeyCode::Char('j') if self.issue_detail_view.show_transitions => {
                self.issue_detail_view.next_transition();
            }
            KeyCode::Up | KeyCode::Char('k') if self.issue_detail_view.show_transitions => {
                self.issue_detail_view.previous_transition();
            }
            KeyCode::Enter if self.issue_detail_view.show_transitions => {
                if let Some(transition) = self.issue_detail_view.selected_transition() {
                    if let Some(issue) = &self.issue_detail_view.issue {
                        let issue_key = issue.key.clone();
                        let transition_id = transition.id.clone();
                        self.jira_client.transition_issue(&issue_key, &transition_id).await?;
                        // Refresh issue details
                        let updated_issue = self.jira_client.get_issue(&issue_key).await?;
                        self.issue_detail_view.set_issue(updated_issue);
                        self.load_transitions(&issue_key).await?;
                    }
                }
                self.issue_detail_view.show_transitions = false;
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_comment_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> Result<()> {
        match key {
            KeyCode::Esc => {
                self.input_view.clear();
                self.mode = AppMode::IssueDetail;
            }
            KeyCode::Enter => {
                if let Some(issue) = &self.issue_detail_view.issue {
                    let comment = self.input_view.get_input();
                    if !comment.is_empty() {
                        self.jira_client.add_comment(&issue.key, comment).await?;
                        // Refresh issue details
                        let updated_issue = self.jira_client.get_issue(&issue.key).await?;
                        self.issue_detail_view.set_issue(updated_issue);
                    }
                }
                self.input_view.clear();
                self.mode = AppMode::IssueDetail;
            }
            KeyCode::Backspace => self.input_view.pop_char(),
            KeyCode::Left => self.input_view.move_cursor_left(),
            KeyCode::Right => self.input_view.move_cursor_right(),
            KeyCode::Char(c) => self.input_view.push_char(c),
            _ => {}
        }
        Ok(())
    }

    async fn handle_edit_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> Result<()> {
        match key {
            KeyCode::Esc => {
                self.input_view.clear();
                self.mode = AppMode::IssueDetail;
            }
            KeyCode::Enter => {
                if let Some(_issue) = &self.issue_detail_view.issue {
                    let new_summary = self.input_view.get_input();
                    if !new_summary.is_empty() {
                        // This is a simplified example - in reality you'd need to construct
                        // the proper update object for Jira
                        // For now, we'll just skip the actual update
                        // let update = IssueUpdate { ... };
                        // self.jira_client.update_issue(&issue.key, update).await?;
                    }
                }
                self.input_view.clear();
                self.mode = AppMode::IssueDetail;
            }
            KeyCode::Backspace => self.input_view.pop_char(),
            KeyCode::Left => self.input_view.move_cursor_left(),
            KeyCode::Right => self.input_view.move_cursor_right(),
            KeyCode::Char(c) => self.input_view.push_char(c),
            _ => {}
        }
        Ok(())
    }

    async fn handle_sprint_selector_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> Result<()> {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('h') => self.show_help = !self.show_help,
            KeyCode::Esc => {
                self.sprint_selector.deactivate();
                self.mode = AppMode::Sprint;
            }
            KeyCode::Down | KeyCode::Char('j') => self.sprint_selector.next(),
            KeyCode::Up | KeyCode::Char('k') => self.sprint_selector.previous(),
            KeyCode::Enter => {
                if let Some(sprint_id) = self.sprint_selector.selected_sprint_id() {
                    self.current_sprint_id = Some(sprint_id);
                    self.load_sprint_issues(sprint_id).await?;
                    self.sprint_selector.deactivate();
                    self.mode = AppMode::Sprint;
                }
            }
            KeyCode::Char('e') => {
                if let Some(sprint) = self.sprint_selector.selected_sprint() {
                    self.input_view = InputView::new(format!("Edit Sprint Name: {}", sprint.name));
                    self.input_view.input = sprint.name.clone();
                    self.input_view.cursor_position = self.input_view.input.len();
                    self.mode = AppMode::EditSprintName;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_edit_sprint_name_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> Result<()> {
        match key {
            KeyCode::Esc => {
                self.input_view.clear();
                self.mode = AppMode::SprintSelector;
            }
            KeyCode::Enter => {
                if let Some(sprint) = self.sprint_selector.selected_sprint() {
                    let new_name = self.input_view.get_input();
                    if !new_name.is_empty() {
                        let update = crate::jira::SprintUpdate {
                            name: Some(new_name.to_string()),
                            ..Default::default()
                        };
                        self.jira_client.update_sprint(sprint.id, &update).await?;
                        self.refresh_sprints().await?;
                    }
                }
                self.input_view.clear();
                self.mode = AppMode::SprintSelector;
            }
            KeyCode::Backspace => self.input_view.pop_char(),
            KeyCode::Left => self.input_view.move_cursor_left(),
            KeyCode::Right => self.input_view.move_cursor_right(),
            KeyCode::Char(c) => self.input_view.push_char(c),
            _ => {}
        }
        Ok(())
    }

    async fn handle_board_selector_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> Result<()> {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('h') => self.show_help = !self.show_help,
            KeyCode::Esc => {
                self.board_selector.deactivate();
                self.mode = AppMode::Sprint;
            }
            KeyCode::Down | KeyCode::Char('j') => self.board_selector.next(),
            KeyCode::Up | KeyCode::Char('k') => self.board_selector.previous(),
            KeyCode::Enter => {
                if let Some(board_id) = self.board_selector.selected_board_id() {
                    self.config.jira.default_board_id = Some(board_id);
                    // Clear sprint data to force reload for new board
                    self.available_sprints.clear();
                    self.current_sprint_id = None;
                    // Load new board's sprint data
                    self.refresh_sprint().await?;
                    self.board_selector.deactivate();
                    self.mode = AppMode::Sprint;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_project_selector_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> Result<()> {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('h') => self.show_help = !self.show_help,
            KeyCode::Esc => {
                self.project_selector.deactivate();
                self.mode = AppMode::Sprint;
            }
            KeyCode::Down | KeyCode::Char('j') => self.project_selector.next(),
            KeyCode::Up | KeyCode::Char('k') => self.project_selector.previous(),
            KeyCode::Enter => {
            if let Some(project) = self.project_selector.selected_project() {
                let project_key = &project.key;
                    // Load boards for the selected project
                    let project_boards = self.jira_client.get_boards().await?
                        .into_iter()
                        .filter(|board| {
                            // Filter boards that belong to this project
                            // This is a simplified check - in reality you might need to check board location or other attributes
                            board.name.contains(&*project_key) || 
                            board.location.as_ref().map_or(false, |loc| loc.project_key.as_deref() == Some(&*project_key))
                        })
                        .collect::<Vec<_>>();
                    
                    if !project_boards.is_empty() {
                        // Update available boards and set the first one as default
                        self.available_boards = project_boards;
                        self.config.jira.default_board_id = Some(self.available_boards[0].id);
                        
                        // Clear sprint data to force reload for new board
                        self.available_sprints.clear();
                        self.current_sprint_id = None;
                        
                        // Load new board's sprint data
                        self.refresh_sprint().await?;
                    }
                    
                    self.project_selector.deactivate();
                    self.mode = AppMode::Sprint;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_help_input(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('h') | KeyCode::Esc => self.show_help = false,
            _ => {}
        }
        Ok(self.should_quit)
    }

    async fn refresh_sprints(&mut self) -> Result<()> {
        if let Some(board_id) = self.config.jira.default_board_id {
            self.available_sprints = self.jira_client.get_board_sprints(board_id).await?;
            self.sprint_selector.set_sprints(self.available_sprints.clone());
        }
        Ok(())
    }

    async fn refresh_sprint(&mut self) -> Result<()> {
        if let Some(board_id) = self.config.jira.default_board_id {
            // Load available sprints if not already loaded
            if self.available_sprints.is_empty() {
                self.available_sprints = self.jira_client.get_board_sprints(board_id).await?;
            }
            
            // If we have a current sprint ID, use it; otherwise find the last (most recent) sprint
            let target_sprint = if let Some(current_id) = self.current_sprint_id {
                self.available_sprints.iter().find(|s| s.id == current_id)
            } else {
                // Start with the last sprint (most recent)
                self.available_sprints.last()
            };
                
            if let Some(sprint) = target_sprint {
                self.current_sprint_id = Some(sprint.id);
                let issues = self.jira_client.get_sprint_issues(board_id, sprint.id).await?;
                self.sprint_view.set_issues(issues, sprint.name.clone(), sprint.goal.clone());
            } else {
                // No sprints available, show empty sprint
                self.sprint_view.set_issues(Vec::new(), "No Sprints Available".to_string(), None);
            }
        }
        Ok(())
    }

    async fn load_backlog(&mut self) -> Result<()> {
        if let Some(board_id) = self.config.jira.default_board_id {
            let issues = self.jira_client.get_backlog(board_id).await?;
            self.backlog_view.set_issues(issues);
        }
        Ok(())
    }

    async fn load_transitions(&mut self, issue_key: &str) -> Result<()> {
        let transitions = self.jira_client.get_transitions(issue_key).await?;
        self.issue_detail_view.set_transitions(transitions);
        Ok(())
    }

    async fn load_sprint_issues(&mut self, sprint_id: u32) -> Result<()> {
        if let Some(board_id) = self.config.jira.default_board_id {
            let issues = self.jira_client.get_sprint_issues(board_id, sprint_id).await?;
            
            // Find the sprint name
            let (sprint_name, sprint_goal) = self.available_sprints
                .iter()
                .find(|s| s.id == sprint_id)
                .map(|s| (s.name.clone(), s.goal.clone()))
                .unwrap_or_else(|| (format!("Sprint {}", sprint_id), None));
            
            self.sprint_view.set_issues(issues, sprint_name, sprint_goal);
        }
        Ok(())
    }
    
    pub async fn initialize(&mut self) -> Result<()> {
        // Load projects if none are available
        if self.available_projects.is_empty() {
            self.available_projects = self.jira_client.get_projects().await.unwrap_or_default();
        }
        
        // Load boards if none are available
        if self.available_boards.is_empty() {
            self.available_boards = self.jira_client.get_boards().await.unwrap_or_default();
        }
        
        // Set default board if not configured but boards are available
        if self.config.jira.default_board_id.is_none() && !self.available_boards.is_empty() {
            self.config.jira.default_board_id = Some(self.available_boards[0].id);
        }
        
        // Load initial sprint data
        self.refresh_sprint().await?;
        Ok(())
    }

    pub fn render(&mut self, f: &mut Frame) {
        if self.show_help {
            self.help_view.render(f, f.size());
            return;
        }

        match self.mode {
            AppMode::AddComment | AppMode::EditIssue | AppMode::EditSprintName => {
                self.render_input_overlay(f);
            }
            _ => {
                self.render_main_layout(f);
            }
        }
    }

    fn render_main_layout(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Tab bar
                Constraint::Min(0),     // Main content
                Constraint::Length(3),  // Status bar
            ])
            .split(f.size());

        // Tab bar
        let titles = vec!["Sprint", "Backlog", "Issue Detail"];
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Jira TUI"))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(match self.mode {
                AppMode::Sprint => 0,
                AppMode::Backlog => 1,
                AppMode::IssueDetail => 2,
                _ => 0,
            });
        f.render_widget(tabs, chunks[0]);

        // Main content
        match self.mode {
            AppMode::Sprint => self.sprint_view.render(f, chunks[1]),
            AppMode::SprintSelector => self.sprint_selector.render(f, chunks[1]),
            AppMode::BoardSelector => self.board_selector.render(f, chunks[1]),
            AppMode::ProjectSelector => self.project_selector.render(f, chunks[1]),
            AppMode::Backlog => self.backlog_view.render(f, chunks[1]),
            AppMode::IssueDetail => self.issue_detail_view.render(f, chunks[1]),
            _ => {}
        }

        // Status bar with contextual keybindings
        self.render_status_bar(f, chunks[2]);
    }

    fn render_input_overlay(&mut self, f: &mut Frame) {
        // Render the main content first
        self.render_main_layout(f);

        // Render input overlay
        let area = centered_rect(60, 20, f.size());
        f.render_widget(Block::default().style(Style::default().bg(Color::Black)), area);
        self.input_view.render(f, area);
    }

    fn render_status_bar(&self, f: &mut Frame, area: Rect) {
        let keybindings = self.get_contextual_keybindings();
        let keybinding_count = keybindings.len();
        
        let keybinding_spans: Vec<Span> = keybindings
            .into_iter()
            .enumerate()
            .flat_map(|(i, (key, desc))| {
                let mut spans = vec![
                    Span::styled(
                        format!(" {}", key),
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(" {}", desc),
                        Style::default().fg(Color::Gray),
                    ),
                ];
                
                // Add separator between keybindings (except for the last one)
                if i < keybinding_count - 1 {
                    spans.push(Span::styled(" │", Style::default().fg(Color::DarkGray)));
                }
                
                spans
            })
            .collect();

        let status_line = Line::from(keybinding_spans);
        let status_bar = Paragraph::new(status_line)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().bg(Color::Black));

        f.render_widget(status_bar, area);
    }

    fn get_contextual_keybindings(&self) -> Vec<(&'static str, &'static str)> {
        let mut bindings = vec![
            ("q", "Quit"),
            ("h", "Help"),
        ];

        match self.mode {
            AppMode::Sprint => {
                bindings.extend_from_slice(&[
                    ("j/k", "Navigate"),
                    ("Enter", "View Issue"),
                    ("r", "Refresh"),
                    ("Tab", "Sprint Selector"),
                    ("B", "Board Selector"),
                    ("P", "Project Selector"),
                    ("s", "Sprint"),
                    ("b", "Backlog"),
                ]);
            }
            AppMode::SprintSelector => {
                bindings.extend_from_slice(&[
                    ("j/k", "Navigate"),
                    ("Enter", "Select Sprint"),
                    ("e", "Edit Sprint"),
                    ("Esc", "Back"),
                ]);
            }
            AppMode::BoardSelector => {
                bindings.extend_from_slice(&[
                    ("j/k", "Navigate"),
                    ("Enter", "Select Board"),
                    ("Esc", "Back"),
                ]);
            }
            AppMode::ProjectSelector => {
                bindings.extend_from_slice(&[
                    ("j/k", "Navigate"),
                    ("Enter", "Select Project"),
                    ("Esc", "Back"),
                ]);
            }
            AppMode::Backlog => {
                bindings.extend_from_slice(&[
                    ("j/k", "Navigate"),
                    ("Enter", "View Issue"),
                    ("r", "Refresh"),
                    ("s", "Sprint"),
                    ("b", "Backlog"),
                ]);
            }
            AppMode::IssueDetail => {
                if self.issue_detail_view.show_transitions {
                    bindings.extend_from_slice(&[
                        ("j/k", "Navigate"),
                        ("Enter", "Apply Transition"),
                        ("Esc", "Back"),
                    ]);
                } else {
                    bindings.extend_from_slice(&[
                        ("c", "Comment"),
                        ("e", "Edit"),
                        ("t", "Transitions"),
                        ("Esc", "Back"),
                    ]);
                }
            }
            AppMode::AddComment => {
                bindings.extend_from_slice(&[
                    ("Enter", "Submit"),
                    ("Esc", "Cancel"),
                    ("←/→", "Move Cursor"),
                ]);
            }
            AppMode::EditIssue => {
                bindings.extend_from_slice(&[
                    ("Enter", "Save"),
                    ("Esc", "Cancel"),
                    ("←/→", "Move Cursor"),
                ]);
            }
            AppMode::EditSprintName => {
                bindings.extend_from_slice(&[
                    ("Enter", "Save"),
                    ("Esc", "Cancel"),
                    ("←/→", "Move Cursor"),
                ]);
            }
            AppMode::Help => {
                bindings.extend_from_slice(&[
                    ("Esc", "Close Help"),
                ]);
            }
        }

        bindings
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
