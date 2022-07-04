mod bluetooth;
mod volume;

use chrono::prelude::*;
use std::process::Command;
use log::info;
use simple_logger::SimpleLogger;

fn datetime() -> String {
    let local = Local::now();

    local.format("%a %d %b %H:%M CLST").to_string()
}

fn final_bar() -> String {
    format!(
        "{} | {} | {}",
        volume::volume().or(Some("".to_string())).unwrap(),
        bluetooth::devices(),
        datetime()
    )
}

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("starting achbar");

    Command::new("xsetroot")
        .arg("-name")
        .arg(final_bar())
        .spawn()
        .expect("Failed to set x root");
}
