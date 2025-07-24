# Jira TUI

A Terminal User Interface (TUI) for Jira Software Cloud, built with Rust and ratatui.

## Features

- 🚀 **Sprint Management**: View active sprints and their issues
- 📋 **Backlog View**: Browse and manage your product backlog
- 🔍 **Issue Details**: View detailed information about issues
- 🔄 **Transitions**: Change issue status with keyboard shortcuts
- 💬 **Comments**: Add comments to issues
- ⚡ **Fast Navigation**: Vim-like keyboard shortcuts
- 🎨 **Intuitive UI**: Clean terminal interface with color coding

## API Compliance

This application is fully aligned with the **Jira Software Cloud REST API** documentation:

### Core API Endpoints
- **Jira Platform API** (`/rest/api/3/`): For core issue management, transitions, and comments
- **Jira Software API** (`/rest/agile/1.0/`): For boards, sprints, epics, and agile-specific features

### Supported Resources
- **Boards**: List and retrieve board information
- **Sprints**: Get board sprints and sprint details
- **Issues**: Retrieve issues from sprints, backlogs, and epics
- **Epics**: List board epics and epic issues
- **Transitions**: Get available transitions and transition issues
- **Comments**: Add comments to issues

### Response Models
All models are properly structured according to the official Jira API schema:
- Pagination support with `maxResults`, `startAt`, `total`, and `isLast`
- Proper field mappings with serde rename attributes
- Support for optional fields and nested objects

## Installation

### Prerequisites
- Rust 1.70+ 
- A Jira Software Cloud instance
- Jira API token

### Building from Source

```bash
git clone <repository-url>
cd jira_tui
cargo build --release
```

## Configuration

On first run, the application will create a configuration file at `~/.config/jira-tui/config.json`:

```json
{
  "jira": {
    "domain": "your-domain.atlassian.net",
    "username": "your-email@example.com",
    "api_token": "your-api-token",
    "default_board_id": null
  },
  "ui": {
    "theme": "default",
    "refresh_interval": 30
  }
}
```

### Getting Your API Token

1. Go to [Atlassian Account Settings](https://id.atlassian.com/manage-profile/security/api-tokens)
2. Click "Create API token"
3. Give it a label and click "Create"
4. Copy the token and add it to your config file

## Usage

```bash
./target/release/jira-tui
```

### Keyboard Shortcuts

#### Global
- `q` - Quit application
- `h` - Toggle help
- `s` - Switch to Sprint view
- `b` - Switch to Backlog view

#### Sprint/Backlog Views
- `j/k` or `↓/↑` - Navigate issues
- `Enter` - View issue details
- `r` - Refresh data

#### Issue Detail View
- `c` - Add comment
- `e` - Edit issue (summary)
- `t` - Show transitions
- `Esc` - Go back

#### Transitions
- `j/k` or `↓/↑` - Navigate transitions
- `Enter` - Apply selected transition
- `Esc` - Cancel

#### Input Fields
- `Enter` - Submit
- `Esc` - Cancel
- `←/→` - Move cursor
- `Backspace` - Delete character

## Architecture

### Project Structure

```
src/
├── jira/
│   ├── mod.rs       # Module exports
│   ├── models.rs    # Jira API models
│   └── client.rs    # HTTP client implementation
├── ui/
│   ├── mod.rs       # UI module exports  
│   ├── app.rs       # Main application logic
│   ├── events.rs    # Event handling
│   └── components/  # UI components
├── config/
│   └── mod.rs       # Configuration management
└── main.rs          # Application entry point
```

### API Client Design

The Jira client is designed with two separate request methods:

- `send_request()`: For Jira Platform API endpoints (`/rest/api/3/`)
- `send_agile_request()`: For Jira Software API endpoints (`/rest/agile/1.0/`)

This separation ensures proper API usage according to Atlassian's documentation.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui) for the terminal UI
- Uses [reqwest](https://github.com/seanmonstar/reqwest) for HTTP requests
- Follows [Jira Software Cloud REST API](https://developer.atlassian.com/cloud/jira/software/rest/intro/) specifications
