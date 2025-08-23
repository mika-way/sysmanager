use std::process::{Command, Stdio};

pub fn update(nonconfirm: bool) {

    //Standart Paketmanager
    println!("\nSystem wird aktualisiert...");
    println!("\n");
    paketmanager(nonconfirm);

    //Flatpak
    if has_flatpak() {
        println!("\nFlatpak wird aktualisiert...");
        println!("\n");
        flatpak(nonconfirm);
    }

    //Yay
    if has_yay() {
        println!("\nYay wird aktualisiert...");
        println!("\n");
        yay(nonconfirm);
    }

    //Snap
    if has_snap() {
        println!("\nSnap wird aktualisiert...");
        println!("\n");
        snap_refresh();
    }
}

// Bekomme den richtgen Paketmanager und führe aus
fn paketmanager(nonconfirm: bool) {

    match get_package_manager() {
        Some(pm) => {
            if pm == "pacman" { pacman(nonconfirm); }
            if pm == "apt" { apt(nonconfirm); }
            if pm == "dnf" { dnf(nonconfirm); }
            if pm == "zypper" { zypper(nonconfirm); }
        },
        None => {
            eprintln!("Fehler: Kein unterstützter Paketmanager gefunden.");
        }
    }
}

//Finde den richtigen Paketmanager
fn get_package_manager() -> Option<&'static str> {
    if Command::new("apt").arg("--version").output().is_ok() {
        return Some("apt");
    }
    if Command::new("dnf").arg("--version").output().is_ok() {
        return Some("dnf");
    }
    if Command::new("pacman").arg("--version").output().is_ok() {
        return Some("pacman");
    }
    if Command::new("zypper").arg("--version").output().is_ok() {
        return Some("zypper");
    }
    None
}

//Überprüfung
fn has_flatpak() -> bool {
    Command::new("flatpak")
        .arg("--version")
        .output()
        .is_ok()
}

fn has_yay() -> bool {
    Command::new("yay")
        .arg("--version")
        .output()
        .is_ok()
}

fn has_snap() -> bool {
    Command::new("snap")
        .arg("--version")
        .output()
        .is_ok()
}

// Paktemanager Funktions
//Pacman (Arch) I use arch btw...
fn pacman(nonconfirm: bool) {
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
        println!("\nPacman wurde aktualisiert.");
    } else {
        eprintln!("\nFehler beim Aktualisieren von Pacman.");
    }
}

//Apt (Ubuntu, Debian)
fn apt(nonconfirm: bool) {
    // Schritt 1: Aktualisieren der Paketlisten
    let mut update_command = Command::new("sudo");
    update_command.arg("apt").arg("update");

    let update_status = update_command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Fehler beim Starten des apt update-Prozesses");

    if !update_status.success() {
        eprintln!("\nFehler beim Aktualisieren der Paketlisten.");
        return; // Frühzeitiges Beenden bei Fehler
    }

    println!("\nPaketlisten erfolgreich aktualisiert.");

    // Schritt 2: Upgrade der Pakete
    let mut upgrade_command = Command::new("sudo");
    upgrade_command.arg("apt").arg("upgrade");

    if nonconfirm {
        upgrade_command.arg("-y");
    }

    let upgrade_status = upgrade_command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Fehler beim Starten des apt upgrade-Prozesses");

    if upgrade_status.success() {
        println!("\napt wurde erfolgreich aktualisiert.");
    } else {
        eprintln!("\nFehler beim Aktualisieren von apt.");
    }
}


//DNF (Fedora)
fn dnf(nonconfirm: bool) {
    let mut command = Command::new("sudo");
    command.arg("dnf").arg("upgrade");

    if nonconfirm {
        command.arg("-y");
    }

    let status = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Fehler beim Starten des dnf-Prozesses");

    if status.success() {
        println!("\ndnf wurde erfolgreich aktualisiert.");
    } else {
        eprintln!("\nFehler beim Aktualisieren von dnf.");
    }
}

//Zypper (OpenSUSE)
fn zypper(nonconfirm: bool) {
    let mut command = Command::new("sudo");
    command.arg("zypper").arg("update");

    if nonconfirm {
        // Die Option für "nicht interaktiv" bei zypper ist --non-interactive
        command.arg("--non-interactive");
    }

    let status = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Fehler beim Starten des zypper-Prozesses");

    if status.success() {
        println!("\nzypper wurde erfolgreich aktualisiert.");
    } else {
        eprintln!("\nFehler beim Aktualisieren von zypper.");
    }
}

//Flatpak

fn flatpak(nonconfirm: bool) {
    let mut command = Command::new("sudo");
    command.arg("flatpak").arg("update");

    if nonconfirm {
        command.arg("-y");
    }

    let status = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Fehler beim Starten des Flatpak-Prozesses");

    if status.success() {
        println!("\nFlatpak wurde aktualisiert.");
    } else {
        eprintln!("\nFehler beim Aktualisieren von Flatpak.");
    }
}

//Yay
fn yay(nonconfirm: bool) {
    let mut command = Command::new("yay");
    command.arg("-Syu");

    if nonconfirm {
        command.arg("--noconfirm");
    }

    let status = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Fehler beim Starten des Yay-Prozesses");

    if status.success() {
        println!("\nYay wurde aktualisiert.");
    } else {
        eprintln!("\nFehler beim Aktualisieren von Yay.");
    }
}

//Snap
fn snap_refresh() {
    let mut command = Command::new("sudo");
    command.arg("snap").arg("refresh");

    let status = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Fehler beim Starten des Snap-Prozesses");

    if status.success() {
        println!("\nSnap-Pakete wurden erfolgreich aktualisiert.");
    } else {
        eprintln!("\nFehler beim Aktualisieren der Snap-Pakete.");
    }
}