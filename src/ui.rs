use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(frame.area());

    let title = Block::default()
        .style(Style::new().red().bold())
        .title(" Arch Network Manager (q to quit) ");
    frame.render_widget(title, chunks[1]);

    let items: Vec<ListItem> = app.wifi_list
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let style = if i == app.selected_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(s.as_str()).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_symbol(">> ");

    frame.render_widget(list, chunks[2]);
}
