mod app;
mod network;
mod ui;

use app::App;
use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::DefaultTerminal;
use std::time::Duration;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    ratatui::run(|terminal| run_app(terminal, &mut app))?;
    Ok(())
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    app.start_scan();
    loop {
        app.update();
        terminal.draw(|frame| ui::render(app, frame))?;

        if event::poll(Duration::from_millis(100))?
            && let event::Event::Key(key) = event::read()?
            && key.kind == event::KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('r') => app.start_scan(),
                KeyCode::Up => app.previous(),
                KeyCode::Char('k') => app.previous(),
                KeyCode::Char('j') => app.next(),
                KeyCode::Down => app.next(),
                _ => {}
            }
        }
    }
}
