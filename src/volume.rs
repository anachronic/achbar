use log::error;
use std::process::Command;

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
