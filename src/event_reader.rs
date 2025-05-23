use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use crossterm::event::{self, Event, KeyEventKind};

use crate::{message::Message, RunMode, SharedState};

pub struct EventReader;

impl EventReader {
    pub fn run(sender: mpsc::Sender<Message>, shared_state: Arc<Mutex<SharedState>>) {
        thread::spawn(move || {
            loop {
                {
                    let lock = shared_state.lock().unwrap();
                    if matches!(lock.run_mode, RunMode::Exit) {
                        break;
                    }
                }
                if event::poll(Duration::from_secs(0)).unwrap() {
                    match event::read().unwrap() {
                        Event::Key(key_event) => {
                            if key_event.kind == KeyEventKind::Press {
                                sender.send(Message::Input(key_event.code)).unwrap();
                            }
                        }
                        Event::Resize(_, _) => sender.send(Message::Redraw).unwrap(),
                        _ => continue,
                    }
                }
            }
            let lock = shared_state.lock().unwrap();
            lock.logger.log("Exiting EventRead thread.");
        });
    }
}
