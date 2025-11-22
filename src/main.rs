mod app;
mod ui;

use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use app::{App, AppMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                AppMode::Menu => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Enter => app.start_test(),
                        KeyCode::Up => app.menu_up(),
                        KeyCode::Down => app.menu_down(),
                        _ => {}
                    }
                }
                AppMode::Test => {
                    match key.code {
                        KeyCode::Esc => app.return_to_menu(),
                        KeyCode::Char(c) => app.type_char(c),
                        KeyCode::Backspace => app.backspace(),
                        _ => {}
                    }
                }
                AppMode::Results => {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter => app.return_to_menu(),
                        _ => {}
                    }
                }
            }
        }
    }
}
