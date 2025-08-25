==SYSMANAGER==

Ein System-Manager, der die Verwaltung deines Linux-Systems vereinfacht und Zeit spart. Dieses Programm wurde in Rust entwickelt und ist mein erstes Projekt in dieser Sprache.

=Features=

Die aktuellen Hauptfunktionen sind:

Update: Unterstützt momentan nur pacman. Zukünftig sollen weitere Paketmanager hinzugefügt werden.

Clear: Löscht den temporären Ordner /tmp, um Speicherplatz freizugeben.

Info: Zeigt dir alle Infos über dein System an.

--nonconfirm: Überspringt die Bestätigungsabfrage für Befehle.

=Installation=

Du benötigst den **Rust-**Toolchain-Manager cargo. Falls du ihn noch nicht installiert hast, führe den folgenden Befehl im Terminal aus:

"curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"

Um das Programm auszuführen, wechsle in den Projektordner sysmanager und gib diesen Befehl ein:

"cargo run --package sysmanager --bin sysmanager"

Nachdem das Programm kompiliert wurde, findest du die ausführbare Datei im Ordner target/debug. Du kannst sie direkt von dort aus starten:

"./target/debug/sysmanager"

Sollte die Ausführung nicht funktionieren, stelle sicher, dass die Datei die notwendigen Berechtigungen hat.

=Hinweis=

Das Projekt ist noch in einem frühen Entwicklungsstadium und sehr experimentell. Es kann zu unbekannten Fehlern kommen.
