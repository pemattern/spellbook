use std::fs::OpenOptions;
use std::io::Write;

pub struct Logger;

impl Logger {
    pub fn log(log: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("/home/paul/Dev/launcher/launcher.log")
            .unwrap();

        let _ = writeln!(file, "{} {}", chrono::Local::now(), log);
    }
}
