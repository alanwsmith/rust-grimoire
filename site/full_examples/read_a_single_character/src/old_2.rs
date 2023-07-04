// this is a basic run with TUI
// proably not going to be using it since it's a fair amount
// of overhead. Looking at `enquirer` instead
//
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph},
    Frame, Terminal,
};

enum InputMode {
    Normal,
}

struct App {
    input: String,
    input_mode: InputMode,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(());
                    }
                    KeyCode::Char('n') => {
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    // KeyCode::Ctrl('c') => {
                    //     return Ok(());
                    // }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());
    let input = Paragraph::new(app.input.as_ref()).block(
        Block::default()
            // .borders(Borders::ALL)
            .title(" That file doesn't exist. Would you like to create it? (y/n) "),
    );
    f.render_widget(input, chunks[1]);
}
