use crate::{app::App, commands};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

pub fn handle_events(app: &mut App) -> Result<()> {
    if !event::poll(Duration::from_millis(200))? {
        return Ok(());
    }

    let Event::Key(key) = event::read()? else {
        return Ok(());
    };

    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Down | KeyCode::Char('j') => app.next(),
        KeyCode::Up | KeyCode::Char('k') => app.previous(),
        KeyCode::Char('v') => app.message = commands::lore_validate(),
        _ => {}
    }

    Ok(())
}
