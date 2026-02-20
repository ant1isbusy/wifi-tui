
use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::{Block, Borders};
use ratatui::style::{Color, Style};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?; // redrawn every time
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let my_block = Block::default()
        .title(" Network Manager ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(my_block, frame.area());
}
