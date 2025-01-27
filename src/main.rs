use std::io;

use config::Config;
use launcher::Launcher;

mod application;
mod config;
mod icon;
mod launcher;
mod logger;
mod watcher;
mod widgets;

fn main() -> io::Result<()> {
    // set_hook(Box::new(move |panic_info| {
    //     ratatui::restore();
    //     let original_hook = take_hook();
    //     original_hook(panic_info);
    // }));
    let config = Config::load();
    let mut terminal = ratatui::init();
    let app_result = Launcher::new(&config).run(&mut terminal);
    ratatui::restore();
    app_result
}
