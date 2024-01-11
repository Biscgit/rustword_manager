use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Color, Modifier, Style},
    style::Stylize,
    widgets::{Block, Borders, BorderType, List, ListItem, Padding, Paragraph, Tabs},
};
use tui_textarea::TextArea;
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
    let color = if app.page_selected { Color::DarkGray } else { Color::Yellow };
    let items: Vec<ListItem> = app
        .template_names
        .items
        .iter()
        .map(|i| {
            ListItem::new(i.0)
                .style(Style::default()
                    .fg(color)
                )
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let color_border = if app.page_selected { Color::DarkGray } else { Color::White };
    let items = List::new(items)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .fg(color_border)
                .title("Templates")
        )
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
        let mut fields = vec![Constraint::Length(4); template.elements.len()];
        fields.push(Constraint::Min(0));
        fields.push(Constraint::Length(3));

        let input_layout = Layout::new(
            Direction::Vertical,
            fields,
        ).split(area);

        // create input fields dynamically
        let all_filled = app.all_fields_filled();
        let fields = app.text_fields.edit_fields.as_mut().unwrap();

        let highlight_index = fields.current().clone().unwrap();
        let items = &mut fields.items;

        for i in 0..template.elements.len() {
            let current = &mut items[i];

            // apply theme
            if i == highlight_index && app.page_selected {
                field_active(current);
            } else {
                field_inactive(current);
            }

            // render widget in spot
            frame.render_widget(
                current.widget(),
                input_layout[i],
            );
        }

        // render insert button
        let last_index = items.len() - 1;
        let confirm_button = items.last_mut().unwrap();

        let mut color = Color::DarkGray;
        if highlight_index == last_index {

            // set button color depending if entry can be inserted
            if all_filled {
                color = Color::LightGreen;
            } else {
                color = Color::LightRed;
            }
        }

        let block = set_border_color(confirm_button, color);
        confirm_button.set_block(block);


        frame.render_widget(
            confirm_button.widget(),
            *input_layout.last().unwrap(),
        )
    }
}

fn set_border_color<'a>(text_field: &TextArea<'a>, color: Color) -> Block<'a> {
    text_field.block()
        .unwrap()
        .clone()
        .style(Style::default().fg(color))
}

fn field_active(text_field: &mut TextArea<'_>) {
    // sets theme to active
    text_field.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));

    // set color to red if empty
    let mut color = Color::White;
    if text_field.is_empty() {
        color = Color::LightRed;
    }

    let block = set_border_color(text_field, color);
    text_field.set_block(block);
}

fn field_inactive(text_field: &mut TextArea<'_>) {
    // modifies block to look inactive
    text_field.set_cursor_line_style(Style::default());
    text_field.set_cursor_style(Style::default());

    let block = set_border_color(text_field, Color::DarkGray);
    text_field.set_block(block);
}