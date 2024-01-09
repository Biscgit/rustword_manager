use std::str::FromStr;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Color, Modifier, Style},
    style::Stylize,
    widgets::{Block, Borders, BorderType, List, ListItem, Padding, Paragraph, Tabs},
};
use crate::app::App;


pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    // main view
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(3),
            Constraint::Min(3),
        ],
    ).split(frame.size());

    // tabs
    let tab_titles = vec!["Credentials", "New Entry", "Templates"];
    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Pages")
        )
        .select(app.page_index.index)
        .highlight_style(
            Style::default()
                .bold()
                .yellow()
        );
    frame.render_widget(tabs, main_layout[0]);

    // select which page to render
    match app.page_index.index {
        0 => page_credentials(frame, app, main_layout[1]),
        1 => page_new_entry(frame, app, main_layout[1]),
        2 => page_templates(frame, app, main_layout[1]),
        _ => {}
    }
}

fn page_credentials(frame: &mut Frame, app: &mut App, area: Rect) {
    // application pages
    let lists_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ],
    ).split(area);

    // password list
    let password_list = Layout::new(
        Direction::Vertical,
        [
            Constraint::Min(1),
            Constraint::Length(3),
        ],
    ).split(lists_layout[0]);

    // entry view
    let items: Vec<ListItem> = app
        .entries_list
        .items
        .iter()
        .map(|i| {
            ListItem::new(i.0)
                .style(Style::default()
                    .fg(Color::Yellow)
                )
        })
        .collect();


    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("List"))
        .highlight_style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" ❱ ");

    frame.render_stateful_widget(items, password_list[0], &mut app.entries_list.state);

    // search field
    frame.render_widget(
        Paragraph::new("Type to search").block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::horizontal(1))
                .title("Search")
        ),
        password_list[1],
    );

    // password content view
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Credentials"),
        lists_layout[1],
    );
}

fn page_new_entry(frame: &mut Frame, app: &mut App, area: Rect) {
    // application pages
    let lists_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ],
    ).split(area);

    // entry view
    let items: Vec<ListItem> = app
        .template_names
        .items
        .iter()
        .map(|i| {
            ListItem::new(i.0)
                .style(Style::default()
                    .fg(Color::Yellow)
                )
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Templates"))
        .highlight_style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" ❱ ");

    frame.render_stateful_widget(items, lists_layout[0], &mut app.template_names.state);

    // Template display
    let mut displayed_text = String::from_str("Select a template to display").unwrap();
    if let Some(index) = app.current_template {
        displayed_text = index.to_string();
    }

    frame.render_widget(
        Paragraph::new(displayed_text).block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("New Entry")
        ),
        lists_layout[1],
    );
}

fn page_templates(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new("In progress..."),
        area,
    );
}