use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, TestMode};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let title = Paragraph::new("Settings")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let mode_text = match app.settings.test_mode {
        TestMode::Words => format!("Words ({})", app.settings.word_count),
        TestMode::Time => format!("Time ({}s)", app.settings.time_limit),
    };
    let word_count_text = format!("{} words", app.settings.word_count);
    let time_limit_text = format!("{} seconds", app.settings.time_limit);
    let lines_text = format!("{} lines", app.settings.lines_to_display);

    let settings_items = vec![
        ("Test Mode", mode_text.as_str()),
        ("Word Count", word_count_text.as_str()),
        ("Time Limit", time_limit_text.as_str()),
        ("Language", app.settings.language.as_str()),
        ("Display Lines", lines_text.as_str()),
    ];

    let items: Vec<ListItem> = settings_items
        .iter()
        .enumerate()
        .map(|(i, (label, value))| {
            let is_selected = i == app.settings_selection;
            
            let content = if is_selected {
                Line::from(vec![
                    Span::styled("→ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled(*label, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::raw(": "),
                    Span::styled(*value, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                ])
            } else {
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(*label, Style::default().fg(Color::White)),
                    Span::raw(": "),
                    Span::styled(*value, Style::default().fg(Color::Gray)),
                ])
            };
            
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Configure Settings"))
        .style(Style::default().fg(Color::White));

    f.render_widget(list, chunks[1]);
}
