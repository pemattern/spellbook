use std::{sync::mpsc, thread};

use nix::sys::inotify::{AddWatchFlags, InitFlags, Inotify};

use crate::{config::Config, message::Message};
use crossterm::event::{self, Event, KeyEventKind};

pub struct FileWatcher;

impl FileWatcher {
    pub fn spawn(sender: mpsc::Sender<Message>) {
        thread::spawn(move || {
            let mut inotify = Self::refresh_inotify();
            loop {
                let Ok(events) = inotify.read_events() else {
                    continue;
                };
                for event in events.iter() {
                    match event.mask {
                        AddWatchFlags::IN_IGNORED => {
                            inotify = Self::refresh_inotify();
                        }
                        AddWatchFlags::IN_ATTRIB => {
                            sender.send(Message::ReloadConfig).unwrap();
                        }
                        _ => {}
                    }
                }
            }
        });
    }
}

impl FileWatcher {
    pub fn refresh_inotify() -> Inotify {
        let inotify = Inotify::init(InitFlags::all()).unwrap();
        let _ = inotify.add_watch(
            Config::get_full_path().as_str(),
            AddWatchFlags::IN_ATTRIB
                | AddWatchFlags::IN_CREATE
                | AddWatchFlags::IN_MODIFY
                | AddWatchFlags::IN_DELETE_SELF
                | AddWatchFlags::IN_MOVE_SELF
                | AddWatchFlags::IN_IGNORED,
        );
        inotify
    }
}

pub struct EventWatcher;

impl EventWatcher {
    pub fn spawn(sender: mpsc::Sender<Message>) {
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
