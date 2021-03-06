use std::process::Command;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

fn device_is_connected(mac: &str) -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("bluetoothctl info {}", mac))
        .output();

    match output {
        Err(_) => false,
        Ok(result) => {
            let device_info = std::str::from_utf8(&result.stdout).unwrap_or("");
            let device_info = device_info.trim();

            device_info
                .split("\n")
                .find(|line| line.trim().eq("Connected: yes"))
                .is_some()
        }
    }
}

pub fn devices() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("bluetoothctl devices | awk '{print $2}'")
        .output();

    match output {
        Err(_) => String::from(""),
        Ok(result) => {
            let devices = std::str::from_utf8(&result.stdout).unwrap_or("");
            let connected_devices = devices
                .trim()
                .split("\n")
                .filter(|device| device_is_connected(device))
                .count();

            format!("Bluetooth: {}", connected_devices)
        }
    }
}

pub fn run_bluetooth_thread(sender: Sender<(&str, String)>) {
    loop {
        sender.send(("bluetooth", devices())).ok();

        thread::sleep(Duration::from_secs(60));
    }
}
