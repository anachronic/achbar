mod bluetooth;
mod volume;

use chrono::prelude::*;
use log::info;
use simple_logger::SimpleLogger;
use std::process::Command;
use std::{thread, time};

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

fn set_root() {
    Command::new("xsetroot")
        .arg("-name")
        .arg(final_bar())
        .spawn()
        .expect("Failed to set x root");
}

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("starting achbar");

    loop {
        set_root();

        thread::sleep(time::Duration::from_secs(30));
    }
}
