use std::io;

use config::Config;
use launcher::Launcher;
use watcher::Watcher;

mod application;
mod config;
mod icon;
mod launcher;
mod logger;
mod watcher;
mod widgets;

fn main() -> io::Result<()> {
    let config = Config::load();
    let mut terminal = ratatui::init();
    Watcher::watch();
    let app_result = Launcher::new(&config).run(&mut terminal);
    ratatui::restore();
    app_result
}
