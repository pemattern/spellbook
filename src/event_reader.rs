use std::{sync::mpsc, thread};

use crossterm::event::{self, Event, KeyEventKind};

use crate::message::Message;

pub struct EventReader;

impl EventReader {
    pub fn listen(sender: mpsc::Sender<Message>) {
        thread::spawn(move || {
            loop {
                match event::read().unwrap() {
                    Event::Key(key_event) => {
                        if key_event.kind == KeyEventKind::Press {
                            sender.send(Message::Input(key_event)).unwrap();
                        }
                    }

                    Event::Resize(_, _) => sender.send(Message::Redraw).unwrap(),

                    _ => continue,
                }
            }
        });
    }
}
