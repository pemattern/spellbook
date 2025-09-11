mod application;
mod config;
mod db;
mod icon;
mod keybind;
mod message;
mod spellbook;
mod widgets;
mod worker;

fn main() -> std::io::Result<()> {
    let (sender, receiver) = std::sync::mpsc::channel();
    worker::FileWatcher::spawn(sender.clone());
    worker::EventWatcher::spawn(sender.clone());
    spellbook::Spellbook::new(receiver).run()
}
