use crate::app::App;
use crate::models::InputMode;
use crate::widgets::TodosTableWidget;
use ratatui::layout::{Alignment, Constraint, Layout, Position};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, StatefulWidget};
use ratatui::Frame;

pub fn render(app: &mut App, frame: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(2),
        Constraint::Min(2),
    ]);

    let [help_area, input_area, todos_table_area] = vertical.areas(frame.area());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                "Press ".into(),
                "q".bold(),
                " to exit, ".into(),
                "n".bold(),
                " to add a new todo".bold(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to cancel, ".into(),
                "Enter".bold(),
                " to save the todo".into(),
            ],
            Style::default(),
        ),
    };

    let text = Text::from(Line::from(msg)).patch_style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, help_area);

    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Green),
        })
        .block(Block::bordered().title("Add a new todo"));
    frame.render_widget(input, input_area);

    match app.input_mode {
        InputMode::Normal => {}
        #[allow(clippy::cast_possible_truncation)]
        InputMode::Editing => frame.set_cursor_position(Position::new(
            input_area.x + app.character_index as u16 + 1,
            input_area.y + 1,
        )),
    }

    // let todos: Vec<ListItem> = app
    //     .todos
    //     .iter()
    //     .enumerate()
    //     .map(|(i, t)| {
    //         let content = Line::from(Span::raw(format!("{i}: {t}")));
    //         ListItem::new(content)
    //     })
    //     .collect();
    // let todos = List::new(todos)
    //     .block(Block::bordered().title("Todos"))
    //     .highlight_style(Style::new().reversed())
    //     .highlight_symbol(">");
    //
    // // let mut buf = frame.buffer_mut();
    // //
    // // StatefulWidget::render(todos, todos_area, &mut buf, &mut app.todos_state);
    //
    // frame.render_stateful_widget(todos, todos_area, &mut app.todos_state);
    let buf = frame.buffer_mut();
    let mut table_state = app.todos_table.todos_state.clone();
    app.todos_table
        .render(todos_table_area, buf, &mut table_state);

    // frame.render_widget(
    //     Paragraph::new(format!(
    //         "This is a tui template. \n\
    //             Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //             Press left and right to increment and decrement the counter respectively.\n\
    //             Counter: {}\n\
    //             Also the todos are: {:?}",
    //         app.counter,
    //         app.todos,
    //     ))
    //         .block(
    //             Block::bordered()
    //                 .title("Template")
    //                 .title_alignment(Alignment::Center)
    //                 .border_type(BorderType::Rounded),
    //         )
    //         .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    //         .centered(),
    //     frame.area(),
    // )
}
