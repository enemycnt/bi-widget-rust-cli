use std::io::Stdout;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
};

use crate::app::App;

pub fn render_app(f: &mut ratatui::Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    // *Layout*
    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(50), Constraint::Percentage(50)].as_ref())
        .horizontal_margin(5)
        .vertical_margin(5)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Black);

    let header_cells = ["Pair", "Last Price", "Change"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));

    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.clone()));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });

    let t = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Market"),
        )
        .highlight_style(selected_style)
        .highlight_symbol("★ ")
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Length(20),
            Constraint::Min(10),
        ]);
    let extra_info =
        Paragraph::new(format!("Pairs recieved: {}", app.streams_count)).block(Block::default());

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(100),
                Constraint::Min(4),
            ]
            .as_ref(),
        )
        .split(rects[0]);

    f.render_stateful_widget(t, chunks[1], &mut app.state);
    f.render_widget(extra_info, chunks[2]);
}
