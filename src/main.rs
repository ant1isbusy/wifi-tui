mod app;
mod ui;
mod network;

use app::App;
use color_eyre::Result;
use ratatui::DefaultTerminal;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    ratatui::run(|terminal| run_app(terminal, &mut app))?;
    Ok(())
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui::render(app, frame))?;
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                use crossterm::event::KeyCode;
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char('k') => app.previous(),
                    KeyCode::Char('j') => app.next(),
                    KeyCode::Down => app.next(),
                    _ => {}
                }
            }
        }
    }
}

