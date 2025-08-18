use std::io::{self};
use std::process::Command;

pub fn clear(noconfirm: bool){
    if noconfirm {
        println!("Achtung: Dieser Befehl löscht temporäre Dateien. Das kann zu einem Verlust von nicht gespeicherten Daten führen.");
        println!("Bist du sicher, dass du fortfahren möchtest? (J/N)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Es ist ein Fehler beim Lesen der Eingabe aufgetreten.");

        if input.trim().to_lowercase() != "j"{
            println!("Der Vorgang wurde abgebrochen!");
            return;
        }
    }
    println!("Löschen der temporären Dateien...");
    let output = Command::new("sudo")
        .arg("rm")
        .arg("-rf")
        .arg("/tmp/*")
        .output()
        .expect("Es ist ein Fehler beim ausführen des Commands.");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("\n");
        println!("Die temporären Dateien wurden gelöscht.");
    } else {
        println!("Fehler beim Löschen der temporären Dateien.");
    }
}