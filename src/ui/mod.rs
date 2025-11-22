mod layout;
mod render_menu;
mod render_test;
mod render_stats;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use crate::app::{App, AppMode};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    layout::render_header(f, chunks[0]);

    match app.mode {
        AppMode::Menu => render_menu::render(f, app, chunks[1]),
        AppMode::Test => render_test::render(f, app, chunks[1]),
        AppMode::Results => render_stats::render(f, app, chunks[1]),
    }

    layout::render_footer(f, app, chunks[2]);
}
