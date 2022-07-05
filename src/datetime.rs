use std::sync::mpsc::Sender;
use std::thread;
use std::time;

use chrono::Local;

pub fn datetime() -> String {
    let local = Local::now();

    local.format("%a %d %b %H:%M CLST").to_string()
}

pub fn run_datetime_thread(sender: Sender<(&str, String)>) {
    loop {
        sender.send(("datetime", datetime())).ok();

        thread::sleep(time::Duration::from_secs(30));
    }
}
