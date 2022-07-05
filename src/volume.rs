use log::error;
use std::io::BufRead;
use std::process::Command;
use std::sync::mpsc::Sender;
use subprocess::Exec;

pub fn volume() -> String {
    let vol = Command::new("pamixer").arg("--get-volume").output();

    match vol {
        Err(_) => {
            error!("Couldn't run pamixer");
            String::from("")
        }
        Ok(output) => {
            let output = std::str::from_utf8(&output.stdout).unwrap_or("");

            format!("Vol: {}%", output.trim())
        }
    }
}

pub fn run_volume_thread(sender: Sender<(&str, String)>) {
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

        sender.send(("volume", volume())).ok();
    }
}
