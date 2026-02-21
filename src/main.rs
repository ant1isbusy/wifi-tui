mod app;
mod network;
mod ui;

use app::{App, InputMode};
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
                KeyCode::Enter => {
                    if app.wifi_list.is_empty() {
                        return Ok(());
                    }

                    match app.input_mode {
                        app::InputMode::Normal => {
                            let net = &app.wifi_list[app.highlighted_index];
                            app.selected_network = Some(net.clone());

                            if net.is_saved {
                                app.connect();
                            } else {
                                app.input_mode = app::InputMode::Editing;
                            }
                        }

                        app::InputMode::Editing => {
                            app.connect();
                            app.input_mode = app::InputMode::Normal;
                        }
                    }
                }
                KeyCode::Char(c) if app.input_mode == InputMode::Editing => {
                    app.password_input.push(c);
                }
                KeyCode::Backspace if app.input_mode == InputMode::Editing => {
                    app.password_input.pop();
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                    app.selected_network = None;
                    app.password_input.clear();
                }
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
