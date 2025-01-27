use std::thread;

use nix::sys::inotify::{AddWatchFlags, InitFlags, Inotify};

use crate::{config::Config, logger::Logger};

#[derive(Debug)]
pub struct Watcher;

impl Watcher {
    pub fn watch() {
        let mut inotify = Self::refresh_inotify();
        thread::spawn(move || loop {
            let Ok(events) = inotify.read_events() else {
                continue;
            };
            for event in events.iter() {
                match event.mask {
                    AddWatchFlags::IN_IGNORED => {
                        inotify = Self::refresh_inotify();
                    }
                    _ => {}
                }
            }
            Logger::log(format!("{:#?}", events).as_str());
        });
    }
}

impl Watcher {
    pub fn refresh_inotify() -> Inotify {
        let inotify = Inotify::init(InitFlags::all()).unwrap();
        inotify
            .add_watch(
                Config::get_path().as_str(),
                AddWatchFlags::IN_ATTRIB
                    | AddWatchFlags::IN_CREATE
                    | AddWatchFlags::IN_MODIFY
                    | AddWatchFlags::IN_DELETE_SELF
                    | AddWatchFlags::IN_MOVE_SELF
                    | AddWatchFlags::IN_IGNORED,
            )
            .unwrap();
        inotify
    }
}
