use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(frame.area());

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(25),
            Constraint::Percentage(45),
        ])
        .split(root[0]);

    draw_artifacts(frame, app, columns[0]);
    draw_relations(frame, app, columns[1]);
    draw_preview(frame, app, columns[2]);
    draw_status(frame, app, root[1]);
}

fn draw_artifacts(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let items: Vec<ListItem> = app
        .artifacts
        .iter()
        .enumerate()
        .map(|(index, artifact)| {
            let line = if index == app.selected {
                Line::from(vec![Span::styled(
                    artifact.label(),
                    Style::default().add_modifier(Modifier::BOLD),
                )])
            } else {
                Line::from(artifact.label())
            };
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(Block::default().title("Artifacts").borders(Borders::ALL));
    frame.render_widget(list, area);
}

fn draw_relations(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let relations = app
        .selected_artifact()
        .map(|artifact| artifact.relations())
        .unwrap_or_default();

    let content = if relations.is_empty() {
        vec![ListItem::new("No relations")]
    } else {
        relations.into_iter().map(ListItem::new).collect()
    };

    let list = List::new(content).block(Block::default().title("Relations").borders(Borders::ALL));
    frame.render_widget(list, area);
}

fn draw_preview(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let text = match app.selected_artifact() {
        Some(artifact) => format!(
            "{}\n{}\nStatus: {}\n\n{}",
            artifact.meta.id, artifact.meta.title, artifact.meta.status, artifact.body
        ),
        None => "No artifact selected".to_string(),
    };

    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Preview").borders(Borders::ALL))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn draw_status(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let status = format!(
        " q/Esc quit | ↑/↓ or j/k navigate | v validate | cwd: {} | {} ",
        app.root.display(),
        app.message
    );

    let paragraph = Paragraph::new(status).block(Block::default().title("Status").borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}
