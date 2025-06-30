use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::path::Path;
use std::fs;

/// Führt einen Git-Befehl aus und gibt das Ergebnis zurück.
/// Gibt Fehler und Ausgaben auf der Konsole aus.
fn run_git_command(args: &[&str]) -> io::Result<()> {
    let git_dir = "/home/torben/.local/share/unsafeToDo"; // oder besser: über `Config`

    let status = Command::new("git")
        .arg("-C")
        .arg(git_dir)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        eprintln!("Git-Befehl fehlgeschlagen: git {}", args.join(" "));
    }
    Ok(())
}

/// Führt `git pull` aus, um aktuelle Änderungen vom Remote zu holen.
pub fn git_pull() -> io::Result<()> {
    println!("🔄 Pulling changes from origin...");
    run_git_command(&["pull", "origin", "main"])
}

/// Führt `git add <file>` aus.
fn git_add(file: &str) -> io::Result<()> {
    run_git_command(&["add", file])
}

/// Führt `git commit -m <msg>` aus.
fn git_commit(message: &str) -> io::Result<()> {
    run_git_command(&["commit", "-m", message])
}

/// Führt `git push` aus.
fn git_push() -> io::Result<()> {
    println!("📤 Pushing changes to origin...");
    run_git_command(&["push", "origin", "main"])
}

/// Führt einen kompletten Synchronisationsvorgang durch.
/// - Pullt neue Änderungen
/// - Fügt die Datei hinzu
/// - Committet sie
/// - Pusht sie
pub fn sync_file(file: &str) -> io::Result<()> {
    git_pull()?;                      // Änderungen vom Server holen
    git_add(file)?;                  // Datei zum Commit hinzufügen
    git_commit("Update todo list")?; // Commit erstellen
    git_push()?;                     // Hochladen
    Ok(())
}

pub fn setup_repo(path: &Path, remote_url: Option<&str>) -> io::Result<()> {
    if path.join(".git").exists() {
        println!("Git-Repo existiert bereits.");
        return Ok(());
    }

    if let Some(remote) = remote_url {
        println!("Klonen von Remote-Repo: {}", remote);
        // `git clone <remote> <path>`
        let status = Command::new("git")
            .args(&["clone", remote, path.to_str().unwrap()])
            .status()?;
        if !status.success() {
            eprintln!("Fehler beim Klonen des Repos");
        }
    } else {
        println!("Initialisiere neues lokales Git-Repo");
        fs::create_dir_all(path)?;
        let status = Command::new("git")
            .args(&["init"])
            .current_dir(path)
            .status()?;
        if !status.success() {
            eprintln!("Fehler bei git init");
        }

        // Leere todos.json erstellen, falls noch nicht vorhanden
        let todos_path = path.join("todos.json");
        if !todos_path.exists() {
            fs::write(&todos_path, "[]")?;
        }

        // Dateien hinzufügen und committen
        Command::new("git")
            .args(&["add", "todos.json"])
            .current_dir(path)
            .status()?;
        Command::new("git")
            .args(&["commit", "-m", "Initial commit"])
            .current_dir(path)
            .status()?;

        // Remote origin setzen, falls URL vorhanden
        if let Some(remote) = remote_url {
            println!("Remote origin hinzufügen: {}", remote);
            let status = Command::new("git")
                .args(&["remote", "add", "origin", remote])
                .current_dir(path)
                .status()?;
            if !status.success() {
                eprintln!("Fehler beim Hinzufügen des Remotes");
            }
        }
    }

    Ok(())
}