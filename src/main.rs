use event_reader::EventReader;
use spellbook::Spellbook;
use watcher::Watcher;

mod application;
mod config;
mod db;
mod event_reader;
mod icon;
mod message;
mod spellbook;
mod watcher;
mod widgets;

fn main() -> std::io::Result<()> {
    let (sender, receiver) = std::sync::mpsc::channel();
    Watcher::run(sender.clone());
    EventReader::listen(sender);
    let app_result = Spellbook::new(receiver).run();
    ratatui::restore();
    app_result
}
