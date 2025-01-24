use std::{fs, io};

use app::App;
use config::Config;

mod app;
mod config;
mod desktop_entry;
mod icons;
mod widgets;

fn main() -> io::Result<()> {
    let toml = fs::read_to_string("./src/launcher.toml").unwrap();
    let config = toml::from_str::<Config>(&toml).unwrap();
    let mut terminal = ratatui::init();
    let app_result = App::new(config).run(&mut terminal);
    ratatui::restore();
    app_result
}
