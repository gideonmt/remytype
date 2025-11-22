use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let title = Paragraph::new("Your Statistics")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let hours = app.user_stats.total_time_seconds / 3600;
    let minutes = (app.user_stats.total_time_seconds % 3600) / 60;
    
    let stats_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Total Tests: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{}", app.user_stats.total_tests),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Average WPM: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:.1}", app.user_stats.average_wpm),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Best WPM: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:.1}", app.user_stats.best_wpm),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Average Accuracy: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:.1}%", app.user_stats.average_accuracy),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Total Words Typed: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{}", app.user_stats.total_words_typed),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Total Time: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{}h {}m", hours, minutes),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter or Esc to return to menu",
            Style::default().fg(Color::Gray),
        )),
    ];

    let stats = Paragraph::new(stats_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Overall Statistics"));
    f.render_widget(stats, chunks[1]);
}
