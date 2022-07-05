mod bluetooth;
mod datetime;
mod volume;

use linked_hash_map::LinkedHashMap;
use log::info;
use simple_logger::SimpleLogger;
use std::process::Command;
use std::sync::mpsc;
use std::thread;

fn reprint_bar(bar: &LinkedHashMap<&str, String>) {
    let fmt = bar
        .values()
        .filter(|str| !str.is_empty())
        .map(|str| str.to_string())
        .collect::<Vec<_>>()
        .join(" | ");

    let mut child = Command::new("xsetroot")
        .arg("-name")
        .arg(fmt)
        .spawn()
        .expect("Failed to set x root");

    child.wait().expect("Couldn't wait for xsetroot");
}

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("starting achbar");

    let mut bar: LinkedHashMap<&str, String> = LinkedHashMap::new();

    bar.insert("volume", volume::volume());
    bar.insert("bluetooth", bluetooth::devices());
    bar.insert("datetime", datetime::datetime());

    let (tx, rx): (mpsc::Sender<(&str, String)>, mpsc::Receiver<(&str, String)>) = mpsc::channel();

    let volume_tx = tx.clone();
    let datetime_tx = tx.clone();
    let bluetooth_tx = tx.clone();

    thread::spawn(move || {
        bluetooth::run_bluetooth_thread(bluetooth_tx);
    });

    thread::spawn(move || {
        datetime::run_datetime_thread(datetime_tx);
    });

    thread::spawn(move || {
        volume::run_volume_thread(volume_tx);
    });

    loop {
        let message = rx.recv();

        if let Ok((module, value)) = message {
            *bar.get_mut(&module).unwrap() = value;

            reprint_bar(&bar);
        }
    }
}
