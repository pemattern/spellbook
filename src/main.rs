mod application;
mod config;
mod db;
mod icon;
mod message;
mod spellbook;
mod widgets;
mod worker;

fn main() -> std::io::Result<()> {
    let now = std::time::Instant::now();
    let (sender, receiver) = std::sync::mpsc::channel();
    worker::FileWatcher::spawn(sender.clone());
    worker::EventWatcher::spawn(sender.clone());
    spellbook::Spellbook::new(receiver, now).run()
}
