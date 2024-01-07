use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, BorderType, List, ListItem, Padding, Paragraph};
use crate::app::App;


pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    // main view
    let main_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ],
    ).split(frame.size());

    // password list
    let password_list = Layout::new(
        Direction::Vertical,
        [
            Constraint::Min(1),
            Constraint::Length(3),
        ],
    ).split(main_layout[0]);

    // entry view
    let items: Vec<ListItem> = app
        .items
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

    frame.render_stateful_widget(items, password_list[0], &mut app.items.state);

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
        main_layout[1],
    );
}
