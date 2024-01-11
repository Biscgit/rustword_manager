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
        2 => page_template_creator(frame, app, main_layout[1]),
        _ => unreachable!()
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
        .block(Block::default()
            .borders(Borders::ALL)
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
            .title("Templates"))
        .highlight_style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" ❱ ");

    frame.render_stateful_widget(items, lists_layout[0], &mut app.template_names.state);

    // Template display
    if let Some(_index) = app.current_template {
        display_template(frame, app, lists_layout[1]);
    } else {
        frame.render_widget(
            Paragraph::new("Select a template to display")
                .block(
                    Block::default()
                        .borders(Borders::NONE)
                        .padding(Padding::uniform(1))
                ),
            lists_layout[1],
        );
    }
}

fn page_template_creator(frame: &mut Frame, _app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new("In progress..."),
        area,
    );
}

fn display_template(frame: &mut Frame, app: &mut App, area: Rect) {
    // display fields for a new entry if any exist else display nothing
    if let Some(template) = app.templates.get(app.current_template.unwrap_or(0)) {
        let mut fields = vec![Constraint::Length(3); template.elements.len()];
        fields.push(Constraint::Min(0));
        fields.push(Constraint::Length(3));

        let input_layout = Layout::new(
            Direction::Vertical,
            fields,
        ).split(area);

        // create input fields dynamically
        for (i, template) in template.elements.iter().enumerate() {
            frame.render_widget(
                Paragraph::new("Input")
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title(template.name.clone())
                    ),
                input_layout[i],
            )
        }

        // render insert button
        frame.render_widget(
            Paragraph::new("Insert")
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                ),
            *input_layout.last().unwrap(),
        )
    }
}