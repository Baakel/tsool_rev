use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crate::app::{App, AppResult};
use crate::db::getters::get_all_todos;
use crate::db::start_db;
use crate::events::{Event, EventHandler};
use crate::handler::handle_key_events;
use crate::tui::Tui;

mod db;
mod models;
mod app;
mod events;
mod handler;
mod ui;
mod tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenvy::dotenv().ok();
    let db = start_db().await?;
    let mut app = App::new(db);
    app.todos = get_all_todos(&app.db).await;

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
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
        }
    }

    tui.exit()?;
    Ok(())
}
