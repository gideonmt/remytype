use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(area);

    // Welcome message
    let welcome = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Welcome to Typing Tester!",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).title("Welcome"));
    f.render_widget(welcome, chunks[0]);

    // Menu options
    let menu_items = vec![
        "Start Test",
        "View Statistics",
        "Settings",
    ];

    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let content = if i == app.menu_selection {
                Line::from(vec![
                    Span::styled("→ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled(*item, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                ])
            } else {
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(*item, Style::default().fg(Color::Gray)),
                ])
            };
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .style(Style::default().fg(Color::White));

    f.render_widget(list, chunks[1]);
}
