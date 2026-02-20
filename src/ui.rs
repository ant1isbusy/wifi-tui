use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // padding
            Constraint::Length(1), // title
            Constraint::Min(0),    // body
        ])
        .split(frame.area());

    let title = Block::default()
        .style(Style::new().red().bold())
        .title(" wifi-tui");
    frame.render_widget(title, chunks[1]);

    if app.is_scanning {
        let loading = Paragraph::new("Searching for Wi-Fi networks...")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );

        frame.render_widget(loading, chunks[2]);
    } else {
        let items: Vec<ListItem> = app
            .wifi_list
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let style = if i == app.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(s.as_str()).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_symbol(">> ");

        frame.render_widget(list, chunks[2]);
    }
}
