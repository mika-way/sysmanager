use std::process::{Command, Stdio};

pub fn update(nonconfirm: bool) {
    let mut command = Command::new("sudo");
    command.arg("pacman").arg("-Syu");

    if !nonconfirm {
        command.arg("--noconfirm");
    }

    let status = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Fehler beim Starten des Pacman-Prozesses");

    if status.success() {
        println!("\nSystem wurde aktualisiert.");
    } else {
        eprintln!("\nFehler beim Aktualisieren des Systems.");
    }
}