use crate::{app::{App, Focus}, commands};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{path::PathBuf, sync::mpsc, time::Duration};

pub struct ReloadWatcher {
    rx: mpsc::Receiver<notify::Result<notify::Event>>,
    _watcher: RecommendedWatcher,
}

impl ReloadWatcher {
    pub fn new(root: PathBuf) -> Result<Self> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = RecommendedWatcher::new(
            move |result| {
                let _ = tx.send(result);
            },
            Config::default(),
        )?;

        let lore_dir = root.join(".lore");
        let watch_target = if lore_dir.exists() { lore_dir } else { root };
        watcher.watch(&watch_target, RecursiveMode::Recursive)?;

        Ok(Self {
            rx,
            _watcher: watcher,
        })
    }

    fn drain(&mut self) -> bool {
        let mut saw_event = false;
        while let Ok(result) = self.rx.try_recv() {
            if result.is_ok() {
                saw_event = true;
            }
        }
        saw_event
    }
}

pub fn handle_events(app: &mut App, watcher: &mut ReloadWatcher) -> Result<()> {
    if watcher.drain() {
        match app.reload() {
            Ok(()) => {}
            Err(error) => app.message = format!("Reload failed: {error}"),
        }
    }

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
        KeyCode::Tab if key.modifiers.contains(KeyModifiers::SHIFT) => app.focus_previous(),
        KeyCode::Tab => app.focus_next(),
        KeyCode::Enter => {
            if matches!(app.focus, Focus::Related) {
                app.open_selected_related();
            } else {
                app.open_related();
            }
        }
        KeyCode::Char('b') => app.back(),
        KeyCode::Char('v') => app.message = commands::lore_validate(),
        _ => {}
    }

    Ok(())
}
