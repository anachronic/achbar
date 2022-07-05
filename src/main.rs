mod bluetooth;
mod volume;

use chrono::prelude::*;
use linked_hash_map::LinkedHashMap;
use log::info;
use simple_logger::SimpleLogger;
use std::io::prelude::*;
use std::process::Command;
use std::sync::mpsc;
use std::{thread, time};
use subprocess::Exec;

fn datetime() -> String {
    let local = Local::now();

    local.format("%a %d %b %H:%M CLST").to_string()
}

fn reprint_bar(bar: &LinkedHashMap<&str, String>) {
    let fmt = bar
        .values()
        .map(|str| str.to_string())
        .collect::<Vec<_>>()
        .join(" | ");

    Command::new("xsetroot")
        .arg("-name")
        .arg(fmt)
        .spawn()
        .expect("Failed to set x root");
}

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("starting achbar");

    let mut bar: LinkedHashMap<&str, String> = LinkedHashMap::new();

    bar.insert(
        "volume",
        volume::volume().or(Some(String::from(""))).unwrap(),
    );
    bar.insert("bluetooth", bluetooth::devices());
    bar.insert("datetime", datetime());

    let (tx, rx): (mpsc::Sender<(&str, String)>, mpsc::Receiver<(&str, String)>) = mpsc::channel();

    let volume_tx = tx.clone();
    let datetime_tx = tx.clone();

    thread::spawn(move || loop {
        datetime_tx.send(("datetime", datetime())).ok();

        thread::sleep(time::Duration::from_secs(30));
    });

    thread::spawn(move || {
        let stream = Exec::shell("pactl subscribe")
            .stream_stdout()
            .expect("cannot open pactl");

        let mut reader = std::io::BufReader::new(stream);

        loop {
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();

            if !buf.starts_with("Event 'change' on sink") {
                continue;
            }

            let vol = volume::volume().or(Some(String::from(""))).unwrap();
            volume_tx.send(("volume", vol)).ok();
        }
    });

    loop {
        let message = rx.recv();

        if let Ok((module, value)) = message {
            *bar.get_mut(&module).unwrap() = value;

            reprint_bar(&bar);
        }
    }
}
