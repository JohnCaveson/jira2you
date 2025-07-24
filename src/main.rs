use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io, time::Duration};
use ui::{App, EventHandler};

mod config;
mod jira;
mod ui;

#[derive(Parser, Debug)]
#[clap(name = "jira-tui")]
struct Opt {
    #[clap(short, long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _opt = Opt::parse();
    let config = config::Config::load()?;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new(config);
    
    // Initialize the app (load boards, sprints, etc.)
    if let Err(e) = app.initialize().await {
        eprintln!("Failed to initialize app: {}", e);
        eprintln!("Please check your configuration and network connectivity.");
    }
    
    let mut event_handler = EventHandler::new(Duration::from_millis(250));
    let res = run_app(&mut terminal, app, &mut event_handler).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    event_handler: &mut EventHandler,
) -> Result<()> {
    loop {
        terminal.draw(|f| app.render(f))?;

        if let Some(event) = event_handler.next().await {
            if app.handle_event(event).await? {
                break;
            }
        }
    }
    Ok(())
}
