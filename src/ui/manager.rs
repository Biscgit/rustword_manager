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

    // create and name tabs
    let color = if app.page_selected { Color::DarkGray } else { Color::White };
    let tab_titles = vec!["Credentials", "New Entry", "Templates"];
    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .fg(color)
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
    // split view of credentials
    let lists_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ],
    ).split(area);

    // left side
    let password_list = Layout::new(
        Direction::Vertical,
        [
            Constraint::Min(1),
            Constraint::Length(3),
        ],
    ).split(lists_layout[0]);

    // create items to be displayed
    let entry_color = if app.page_selected { Color::DarkGray } else { Color::Yellow };
    let items: Vec<ListItem> = app
        .entries_list
        .items
        .iter()
        .map(|i| {
            ListItem::new(i.0)
                .style(Style::default().fg(entry_color))
        })
        .collect();

    // create a list from all list items and highlight the currently selected one
    let border_color = if app.page_selected { Color::DarkGray } else { Color::White };
    let items = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .fg(border_color)
            .title("List"))
        .highlight_style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" ❱ ");

    frame.render_stateful_widget(items, password_list[0], &mut app.entries_list.state);

    // search field
    let search_bar = &mut app.text_fields.search_bar;
    search_bar.set_placeholder_text("Type to search");
    search_bar.set_block(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1))
            .fg(border_color)
            .title("Search")
    );

    // set bar color to blue if search active
    if !search_bar.is_empty() {
        let block = set_border_color(search_bar, Color::LightBlue);
        search_bar.set_block(block);
    }

    frame.render_widget(
        search_bar.widget(),
        password_list[1],
    );

    // right side: show contents if something selected
    if app.current_entry.is_some() {
        render_credentials(frame, app, lists_layout[1]);
    } else {
        frame.render_widget(
            Paragraph::new("Select an entry to display")
                .block(
                    Block::default()
                        .borders(Borders::NONE)
                        .padding(Padding::uniform(1))
                ),
            lists_layout[1],
        );
    }
}

fn render_credentials(frame: &mut Frame, app: &mut App, area: Rect) {
    // function for rendering selected credentials
    if let Some(entries) = &app.current_entry {
        // create all fields in a layout
        let mut fields = vec![Constraint::Length(4); entries.items.len()];
        fields.push(Constraint::Min(0));

        let credentials_layout = Layout::new(
            Direction::Vertical,
            fields,
        ).split(area);

        // fill fields with content and highlight
        for (entry, (index, field)) in entries.items.iter().zip(credentials_layout.iter().enumerate()) {
            let color = if index == entries.current_index().unwrap() && app.page_selected
            { Color::White } else { Color::DarkGray };

            frame.render_widget(
                Paragraph::new(entry.1)
                    .block(Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .fg(color)
                        .title(entry.0)
                    ),
                *field,
            )
        }
    }
}

fn page_new_entry(frame: &mut Frame, app: &mut App, area: Rect) {
    // split view of templates
    let lists_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ],
    ).split(area);

    // create items to be displayed
    let color = if app.page_selected { Color::DarkGray } else { Color::Yellow };
    let items: Vec<ListItem> = app
        .templates
        .items
        .iter()
        .map(|t| {
            ListItem::new(t.name.clone())
                .style(Style::default().fg(color))
        })
        .collect();

    // create a list from all list items and highlight the currently selected one
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

    frame.render_stateful_widget(items, lists_layout[0], &mut app.templates.state);

    // right side: show template fields if something selected
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


fn display_template(frame: &mut Frame, app: &mut App, area: Rect) {
    // function for rendering input fields of selected template
    if let Some(template) = app.templates.items.get(app.current_template.unwrap_or(0)) {
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

        // set highlight here because of (i)mutable re-use
        let highlight_index = fields.current_index().unwrap();
        let items = &mut fields.items;

        // create all inputs according to provided template
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

        // apply button style
        let block = set_border_color(confirm_button, color);
        confirm_button.set_block(block);

        frame.render_widget(
            confirm_button.widget(),
            *input_layout.last().unwrap(),
        )
    }
}

fn page_template_creator(frame: &mut Frame, _app: &mut App, area: Rect) {
    // placeholder for last page
    frame.render_widget(
        Paragraph::new("In progress..."),
        area,
    );
}

fn set_border_color<'a>(text_field: &TextArea<'a>, color: Color) -> Block<'a> {
    // changes border color from a TextArea and returns new border
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