use crate::app::App;
use crate::models::{InputMode, InputType};
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph, StatefulWidget, Widget, Wrap};
use ratatui::Frame;

pub fn render(app: &mut App, frame: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ]);

    let [help_area, input_area, goals_area, todos_table_area, errors_area] =
        vertical.areas(frame.area());

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
            InputMode::Editing => match app.input_type {
                InputType::Goal => Style::default().fg(Color::Yellow),
                InputType::Todo => Style::default().fg(Color::Green),
                _ => Style::default().fg(Color::Magenta),
            },
        })
        .block(match app.input_type {
            InputType::Goal => Block::bordered().title("Add today's goal"),
            InputType::Todo => Block::bordered().title("Add a new todo"),
            _ => Block::bordered().title("Add a new something"),
        });
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
    // let mut table_state = app.todos_state.clone();
    app.todos_table
        .render(todos_table_area, buf, &mut app.todos_state);

    app.goal_widget.render(goals_area, buf);

    let error_text = Text::from(Line::from(app.errors.clone())).patch_style(Style::default().red());
    let error_message = Paragraph::new(error_text).wrap(Wrap { trim: false });
    frame.render_widget(error_message, errors_area);
}
