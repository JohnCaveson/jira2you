use crossterm::event::{self, KeyCode, KeyModifiers};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyCode, KeyModifiers),
    Tick,
    Quit,
}

pub struct EventHandler {
    sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let event_sender = sender.clone();

        let handler = tokio::spawn(async move {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).unwrap_or(false) {
                    match event::read().unwrap() {
                        event::Event::Key(key) => {
                            if key.kind == event::KeyEventKind::Press {
                                let _ = event_sender.send(Event::Key(key.code, key.modifiers));
                            }
                        }
                        _ => {}
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    let _ = event_sender.send(Event::Tick);
                    last_tick = Instant::now();
                }
            }
        });

        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }

    pub fn stop(self) {
        self.handler.abort();
    }
}
