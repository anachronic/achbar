mod bluetooth;

use chrono::prelude::*;
use std::process::Command;

fn datetime() -> String {
    let local = Local::now();

    local.format("%a %d %b %H:%M CLST").to_string()
}

fn final_bar() -> String {
    format!("{} | {}", bluetooth::devices(), datetime())
}

fn main() {
    Command::new("xsetroot")
        .arg("-name")
        .arg(final_bar())
        .spawn()
        .expect("Failed to set x root");
}
