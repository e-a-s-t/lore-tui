use crate::app::{App, Focus};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
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
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ])
        .split(root[0]);

    draw_artifacts(frame, app, columns[0]);
    draw_relations(frame, app, columns[1]);
    draw_preview(frame, app, columns[2]);
    draw_status(frame, app, root[1]);
}

fn draw_artifacts(frame: &mut Frame, app: &App, area: Rect) {
    let features = app
        .artifacts
        .iter()
        .enumerate()
        .filter(|(_, artifact)| artifact.is_feature());
    let items: Vec<ListItem> = features
        .map(|(index, artifact)| {
            let label = format!(
                "{} [{}]  {}",
                artifact.meta.id, artifact.meta.status, artifact.meta.title
            );
            let line = if index == app.feature_selected {
                Line::from(vec![Span::styled(
                    label,
                    Style::default().add_modifier(Modifier::BOLD),
                )])
            } else {
                Line::from(label)
            };
            ListItem::new(line)
        })
        .collect();

    let title = if matches!(app.focus, Focus::Features) {
        "Features*"
    } else {
        "Features"
    };
    let list = List::new(items).block(Block::default().title(title).borders(Borders::ALL));
    frame.render_widget(list, area);
}

fn draw_relations(frame: &mut Frame, app: &App, area: Rect) {
    let mut items = Vec::new();
    let mut related_index = 0;
    if let Some(artifact) = app.selected_artifact() {
        for (group, ids) in artifact.relation_groups() {
            if group == "Features" {
                continue;
            }
            items.push(ListItem::new(Line::from(Span::styled(
                group,
                Style::default().add_modifier(Modifier::BOLD),
            ))));
            for id in ids.iter() {
                let label = format!("  {id}");
                let line = if matches!(app.focus, Focus::Related)
                    && related_index == app.related_selected
                {
                    Line::from(vec![Span::styled(
                        label,
                        Style::default().add_modifier(Modifier::BOLD),
                    )])
                } else {
                    Line::from(label)
                };
                items.push(ListItem::new(line));
                related_index += 1;
            }
        }
    }

    if items.is_empty() {
        items.push(ListItem::new("No relations"));
    }

    let title = if matches!(app.focus, Focus::Related) {
        "Related*"
    } else {
        "Related"
    };
    let list = List::new(items).block(Block::default().title(title).borders(Borders::ALL));
    frame.render_widget(list, area);
}

fn draw_preview(frame: &mut Frame, app: &App, area: Rect) {
    let paragraph = Paragraph::new(app.preview.as_str())
        .block(Block::default().title("Preview").borders(Borders::ALL))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    let status = format!(
        " Tab switch pane | ↑/↓ navigate | Enter open | b back | v validate | cwd: {} | {} ",
        app.root.display(),
        app.message
    );

    let paragraph =
        Paragraph::new(status).block(Block::default().title("Status").borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}
