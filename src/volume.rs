use std::process::Command;
use log::error;

pub fn volume() -> Option<String> {
    let vol = Command::new("pamixer")
        .arg("--get-volume")
        .output();


    match vol {
        Err(_) => {
            error!("Couldn't run pamixer");
            None
        },
        Ok(output) => {
            let output = std::str::from_utf8(&output.stdout).unwrap_or("");

            let fmt = format!("Vol: {}%", output.trim());
            Some(fmt)
        }
    }
}
