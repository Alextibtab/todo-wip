use std::{sync::mpsc, thread};

use ratatui::crossterm::event::{
    self, Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent,
};

use crate::app::AppResult;

pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || loop {
                match event::read().expect("unable to read event") {
                    CrosstermEvent::Key(e) => {
                        if e.kind == KeyEventKind::Press {
                            sender.send(Event::Key(e))
                        } else {
                            Ok(())
                        }
                    }
                    CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                    CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                    CrosstermEvent::FocusGained => Ok(()),
                    CrosstermEvent::FocusLost => Ok(()),
                    CrosstermEvent::Paste(_) => unimplemented!(),
                }
                .expect("failed to send terminal event")
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }
}
