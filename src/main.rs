use crate::app::{App, AppResult};
use crate::db::start_db;
use crate::events::{Event, EventHandler};
use crate::handler::handle_key_events;
use crate::tui::Tui;
use config::init_configs;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

mod app;
mod config;
mod db;
mod events;
mod handler;
mod models;
mod tui;
mod ui;
mod widgets;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenvy::dotenv().ok();
    init_configs();
    let db = start_db().await?;
    let mut app = App::new(db).await;
    app.get_todays_goal().await;
    app.reload_todos().await;
    app.todos_table.populate_table();
    app.goal_widget.populate_goal();
    app.dailies_table.populate_dailies();

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key) => handle_key_events(key, &mut app).await?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
