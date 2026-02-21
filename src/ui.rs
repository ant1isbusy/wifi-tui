use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
};

use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // padding
            Constraint::Length(1), // title
            Constraint::Min(0),    // body
        ])
        .split(area);

    let title = Paragraph::new(" wifi-tui").style(Style::new().red().bold());
    frame.render_widget(title, chunks[1]);

    if app.is_scanning {
        let loading = Paragraph::new("Searching for Wi-Fi networks...")
            .style(Style::default().fg(Color::Yellow).bold())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );
        frame.render_widget(loading, chunks[2]);
    } else {
        let body_chunks = if app.selected_network.is_some() {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(chunks[2])
        } else {
            Layout::default()
                .constraints([Constraint::Percentage(100)])
                .split(chunks[2])
        };

        let header = Row::new(vec![
            Cell::from(format!("{:<20.20}", "SSID")),
            Cell::from(" "),
            Cell::from("SIGNAL"),
            Cell::from("SECURITY"),
        ])
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

        let rows = app.wifi_list.iter().enumerate().map(|(i, net)| {
            let style = if i == app.highlighted_index {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(format!(" {:<20.20}", net.ssid)),
                Cell::from(if net.is_saved { "." } else { " " }),
                Cell::from(format!(" {}%", net.signal)),
                Cell::from(format!(" {}", net.security)),
            ])
            .style(style)
        });

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(30),
                Constraint::Length(1),
                Constraint::Length(8),
                Constraint::Min(10),
            ],
        )
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Available Networks "),
        )
        .highlight_symbol(">> ")
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        frame.render_widget(table, body_chunks[0]);

        if let Some(net) = &app.selected_network {
            let side_pane_block = Block::default()
                .title(" Connection Details ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow));

            let pane_area = side_pane_block.inner(body_chunks[1]);
            frame.render_widget(side_pane_block, body_chunks[1]);

            let pane_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(4),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ])
                .split(pane_area);

            let info_text = format!(
                "SSID: {}\nSignal: {}%\nSecurity: {}",
                net.ssid, net.signal, net.security
            );
            let info = Paragraph::new(info_text).style(Style::default());
            frame.render_widget(info, pane_chunks[0]);

            let masked_password: String = "*".repeat(app.password_input.len());
            let input_style = match app.input_mode {
                crate::app::InputMode::Editing => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            };

            let password_box = Paragraph::new(masked_password).style(input_style).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Password ")
                    .border_type(BorderType::Plain),
            );
            frame.render_widget(password_box, pane_chunks[1]);

            if let crate::app::InputMode::Editing = app.input_mode {
                frame.set_cursor_position((
                    pane_chunks[1].x + app.password_input.len() as u16 + 1,
                    pane_chunks[1].y + 1,
                ));
            }
        }
    }
}
