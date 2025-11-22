use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(5),
        ])
        .split(area);

    // Timer/Progress
    let progress_text = if let Some(start) = app.start_time {
        let elapsed = start.elapsed().as_secs();
        format!("Time: {}s | Progress: {}/{}", elapsed, app.current_pos, app.test_text.len())
    } else {
        "Press any key to start...".to_string()
    };

    let progress = Paragraph::new(progress_text)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Progress"));
    f.render_widget(progress, chunks[0]);

    // Text display with highlighting
    let mut spans = Vec::new();
    
    for (i, ch) in app.test_text.chars().enumerate() {
        let style = if i < app.current_pos {
            // Already typed
            if i < app.current_input.len() {
                let typed = app.current_input.chars().nth(i).unwrap();
                if typed == ch {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red).add_modifier(Modifier::UNDERLINED)
                }
            } else {
                Style::default().fg(Color::Green)
            }
        } else if i == app.current_pos {
            // Current position
            Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD)
        } else {
            // Not yet typed
            Style::default().fg(Color::Gray)
        };
        
        spans.push(Span::styled(ch.to_string(), style));
    }

    let text_display = Paragraph::new(Line::from(spans))
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Type this text"));
    f.render_widget(text_display, chunks[1]);

    // Current input display
    let input_display = Paragraph::new(app.current_input.as_str())
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Your input"));
    f.render_widget(input_display, chunks[2]);
}
