use std::{fs, io};

use config::Config;
use launcher::Launcher;

mod application;
mod config;
mod icon;
mod launcher;
mod widgets;

fn main() -> io::Result<()> {
    let toml = fs::read_to_string("/home/paul/Dev/launcher/src/launcher.toml").unwrap();
    let config = toml::from_str::<Config>(&toml).unwrap();
    let mut terminal = ratatui::init();
    let app_result = Launcher::new(config).run(&mut terminal);
    ratatui::restore();
    app_result
}
