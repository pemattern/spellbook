use std::io;

use launcher::Launcher;

mod application;
mod config;
mod icon;
mod launcher;
mod logger;
mod widgets;

fn main() -> io::Result<()> {
    let app_result = Launcher::new().run();
    ratatui::restore();
    app_result
}
