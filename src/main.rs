use std::{io, sync::mpsc, time::Instant};

use input_reader::EventReader;
use launcher::Launcher;
use watcher::Watcher;

mod application;
mod config;
mod icon;
mod input_reader;
mod launcher;
mod logger;
mod message;
mod watcher;
mod widgets;

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let (sender, receiver) = mpsc::channel();
    Watcher::new(sender.clone());
    EventReader::listen(sender);
    let app_result = Launcher::new(receiver).run(start_time);
    ratatui::restore();
    app_result
}
