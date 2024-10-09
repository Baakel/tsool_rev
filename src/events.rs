use std::time::Duration;
use futures::{FutureExt, StreamExt};
use ratatui::crossterm::event::{EventStream, KeyEvent, MouseEvent};
use crossterm::event::{Event as CrosstermEvent};
use tokio::sync::mpsc;
use crate::app::AppResult;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();
        // this is probably to not drop the last sender and keep the channel open?
        let _sender = sender.clone();
        let handler = tokio::spawn(async move {
            let mut reader = EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    _ = _sender.closed() => {
                        break;
                    }
                    _ = tick_delay => {
                        _sender.send(Event::Tick).unwrap();
                    }
                    Some(Ok(evt)) = crossterm_event => {
                        match evt {
                            CrosstermEvent::Key(key) => {
                                if key.kind == crossterm::event::KeyEventKind::Press {
                                    _sender.send(Event::Key(key)).unwrap();
                                }
                            },
                            CrosstermEvent::Mouse(mouse) => {
                                _sender.send(Event::Mouse(mouse)).unwrap();
                            },
                            CrosstermEvent::Resize(x, y) => {
                                _sender.send(Event::Resize(x, y)).unwrap();
                            },
                            CrosstermEvent::FocusLost => {},
                            CrosstermEvent::FocusGained => {},
                            CrosstermEvent::Paste(_) => {},
                        }
                    }
                }
            }
        });
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub async fn next(&mut self) -> AppResult<Event> {
        self.receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "This is an io error",
            )))
    }
}