use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::{App, AppMode};

pub fn render_header(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("RemyType")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

pub fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let help_text = match app.mode {
        AppMode::Menu => "[↑/↓] Navigate | [Enter] Start | [q] Quit",
        AppMode::Test => "[Esc] Cancel | Type to test your speed!",
        AppMode::Results => "[Enter/Esc] Return to menu",
    };

    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, area);
}
