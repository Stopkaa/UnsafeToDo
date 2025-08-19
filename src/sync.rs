use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Output};
use crate::display::{display_single_todo, display_todo_list};
use crate::priority::Priority;
use crate::todo;
use crate::todo::{Todo, TodoBuilder, TodoList};

pub struct GitRepo {
    path: PathBuf,
}

impl GitRepo {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        //println!("Initializing GitRepo with path: {}", path.display());
        GitRepo { path }
    }

    fn run_git_command(&self, args: &[&str]) -> io::Result<()> {
        let output: Output = Command::new("git")
            .arg("-C")
            .arg(&self.path)
            .args(args)
            .output()?;

        //zum debuggen
        if !output.status.success() {
            //eprintln!("Git error: git {}", args.join(" "));
            //eprintln!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            //eprintln!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }

    fn choose_merge_strategy(&self) -> io::Result<String> {
        println!("Merge-Konflikt erkannt. Bitte wählen:");
        println!("1) Lokale Änderungen behalten");
        println!("2) Remote Änderungen übernehmen");
        println!("3) Zusammenführen");

        print!("Deine Wahl (1/2/3): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    pub fn handle_merge_conflict(&self) -> io::Result<()> {
        let path = self.path.join("todos.json");
        let file = fs::read_to_string(&path)?;

        let resolved_lines = self.resolve_conflicts(file)?;

        // Overwrite file with resolved content
        fs::write(&path, resolved_lines.join("\n"))?;

        println!("Merge conflicts resolved. You can now commit the changes.");
        Ok(())
    }

    fn resolve_conflicts(&self, content: String) -> io::Result<Vec<String>> {
        let mut result = Vec::new();
        let mut lines = content.lines();

        while let Some(line) = lines.next() {
            if line.starts_with("<<<<<<< ") {
                let mut head_block = Vec::new();
                let mut incoming_block = Vec::new();

                // Collect HEAD section
                while let Some(l) = lines.next() {
                    if l.starts_with("=======") {
                        break;
                    }
                    head_block.push(l.to_string());
                }

                // Collect incoming section
                while let Some(l) = lines.next() {
                    if l.starts_with(">>>>>>>") {
                        break;
                    }
                    incoming_block.push(l.to_string());
                }

                // Prompt user for choice
                println!("\nMerge conflict detected:");
                //println!("Local version:");

                //for l in &head_block {
                //    println!("    {}", l);
                //}

                //println!("Incoming version:");
                //for l in &incoming_block {
                //    println!("    {}", l);
                //}

                let local_todos = todo::todos_from_json_lines(&head_block);
                let incoming_todos = todo::todos_from_json_lines(&incoming_block);

                println!("Local version:");
                for todo in &local_todos {
                    display_single_todo(todo);
                }

                println!("Incoming version:");
                for todo in &incoming_todos {
                    display_single_todo(todo);
                }

                println!("Which version do you want to keep?");
                println!("1) Local version");
                println!("2) Incoming version");
                println!("3) Both");

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                match input.trim() {
                    "1" => result.extend(head_block),
                    "2" => result.extend(incoming_block),
                    "3" => {
                        result.extend(head_block);
                        result.extend(incoming_block);
                    }
                    _ => {
                        println!("Invalid choice. Skipping this conflict.");
                    }
                }
            } else {
                result.push(line.to_string());
            }
        }

        Ok(result)
    }



    pub fn handle_merge_conflict_old(&self) -> io::Result<()> {
        let todos_path = self.path.join("todos.json");

        // Lokale Datei einlesen (ganze Datei als mehrere JSON-Zeilen)
        let local_list = TodoList::load().unwrap_or_else(|_| TodoList::new());

        // Remote Datei als String laden
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.path)
            .args(&["show", "origin/main:todos.json"])
            .output()?;

        if !output.status.success() {
            eprintln!("Konnte remote todos.json nicht laden.");
            return Err(io::Error::new(io::ErrorKind::Other, "Remote load failed"));
        }
        let remote_content = String::from_utf8_lossy(&output.stdout);

        // Remote Liste Zeile für Zeile parsen
        let mut remote_list = TodoList::new();
        for line in remote_content.lines() {
            if !line.trim().is_empty() {
                if let Ok(todo) = serde_json::from_str::<Todo>(line) {
                    remote_list.add(todo);
                }
            }
        }
        
        //listen zeigen
        display_todo_list(&local_list);
        display_todo_list(&remote_list);

        // Auswahl abfragen
        let choice = self.choose_merge_strategy()?;

        // Merge der Todo-Listen basierend auf Auswahl
        let merged_list = match choice.as_str() {
            "1" => local_list,
            "2" => remote_list,
            "3" => self.merge_todo_lists(local_list, remote_list),
            _ => {
                eprintln!("Ungültige Auswahl. Merge abgebrochen.");
                return Err(io::Error::new(io::ErrorKind::Other, "Invalid merge choice"));
            }
        };

        // Zusammengeführte Liste speichern (jede Todo als JSON-Zeile)
        let mut file = fs::File::create(&todos_path)?;
        for todo in merged_list.todos.iter() {
            writeln!(file, "{}", serde_json::to_string(todo)?)?;
        }

        println!("Merge erfolgreich abgeschlossen. Du kannst jetzt committen.");

        Ok(())
    }
    
    fn merge_todo_lists(&self, local: TodoList, remote: TodoList) -> TodoList {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        let mut merged = TodoList::new();

        //alle hinzufuegen wenn nicht komplette zeile gleich
        for todo in local.todos.into_iter().chain(remote.todos.into_iter()) { 
            let key = serde_json::to_string(&todo).unwrap_or_default();
            if set.insert(key) {
                merged.add(todo);
            }
        }

        merged
    }

    pub fn pull(&self) -> io::Result<()> {
        println!("Pulling changes from origin...");
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.path)
            .args(&["pull", "--no-rebase", "origin", "main"])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        //println!("Git pull stdout:\n{}", stdout);

        if output.status.success() {
            //println!("Pull erfolgreich.");
            Ok(())
        } else {
            eprintln!("Git pull stderr:\n{}", stderr);

            let conflict_detected = stdout.contains("CONFLICT")
                || stderr.contains("CONFLICT")
                || stdout.contains("Automatic merge failed")
                || stderr.contains("Automatic merge failed");

            if conflict_detected {
                eprintln!("Merge-Konflikt erkannt.");
                self.handle_merge_conflict()?;
                Ok(())
            } else {
                eprintln!("Git pull Fehler:\n{}", stderr);
                Err(io::Error::new(io::ErrorKind::Other, "Git pull failed"))
            }
        }
    }

    pub fn add(&self, file: &str) -> io::Result<()> {
        self.run_git_command(&["add", file])
    }

    pub fn commit(&self, message: &str) -> io::Result<()> {
        self.run_git_command(&["commit", "-m", message])
    }

    pub fn push(&self) -> io::Result<()> {
        println!("Pushing changes to origin...");
        self.run_git_command(&["push", "origin", "main"])
    }

    pub fn sync_file(&self, file: &str) -> io::Result<()> {
        self.add(file)?;
        self.commit("Update todo list manually?")?;
        self.pull()?;
        self.add(file)?;
        self.commit("Update todo list")?;
        self.push()?;
        Ok(())
    }

    pub fn setup(&self, remote_url: Option<&str>) -> io::Result<()> {
        if self.path.join(".git").exists() {
            println!("Git repository already exists.");
            return Ok(());
        }

        if let Some(remote) = remote_url {
            println!("Cloning from remote repository: {}", remote);
            let status = Command::new("git")
                .args(&["clone", remote, self.path.to_str().unwrap()])
                .status()?;
            if !status.success() {
                eprintln!("Failed to clone repository.");
            }
        } else {
            println!("Initializing new local Git repository...");
            fs::create_dir_all(&self.path)?;
            let status = Command::new("git")
                .args(&["init"])
                .current_dir(&self.path)
                .status()?;
            if !status.success() {
                eprintln!("Failed to initialize repository.");
            }

            let todos_path = self.path.join("todos.json");
            if !todos_path.exists() {
                fs::write(&todos_path, "[]")?;
            }

            Command::new("git")
                .args(&["add", "todos.json"])
                .current_dir(&self.path)
                .status()?;
            Command::new("git")
                .args(&["commit", "-m", "Initial commit"])
                .current_dir(&self.path)
                .status()?;

            if let Some(remote) = remote_url {
                println!("Adding remote origin: {}", remote);
                let status = Command::new("git")
                    .args(&["remote", "add", "origin", remote])
                    .current_dir(&self.path)
                    .status()?;
                if !status.success() {
                    eprintln!("Failed to add remote origin.");
                }
            }
        }

        Ok(())
    }


    pub fn demo_merge(&self) -> std::io::Result<()> {
        use chrono::{Utc, NaiveDate};

        // Lokale Todos mit Builder
        let mut local = TodoList::new();
        local.add(
            TodoBuilder::new()
                .id(1)  // falls nötig, musst du id auch im Builder als Methode ergänzen
                .title("Local Task 1")
                .description(Some("Lokale Aufgabe".to_string()))
                .finished(false)
                .priority(Priority::Medium)
                .due_date(NaiveDate::from_ymd(2025, 7, 1))
                .build()
                .unwrap()
        );
        local.add(
            TodoBuilder::new()
                .id(2)
                .title("Common Task")
                .finished(true)
                .priority(Priority::Low)
                .build()
                .unwrap()
        );

        // Remote Todos mit Builder
        let mut remote = TodoList::new();
        remote.add(
            TodoBuilder::new()
                .id(2)
                .title("Common Task")
                .description(Some("Remote Beschreibung".to_string()))
                .finished(false)
                .priority(Priority::High)
                .due_date(NaiveDate::from_ymd(2025, 7, 10))
                .build()
                .unwrap()
        );
        remote.add(
            TodoBuilder::new()
                .id(3)
                .title("Remote Task 3")
                .finished(false)
                .priority(Priority::Low)
                .build()
                .unwrap()
        );
        
        fn print_list(label: &str, list: &TodoList) {
            println!("--- {} ---", label);
            for todo in &list.todos {
                println!(
                    "ID: {}, Title: {}, Finished: {}, Priority: {:?}",
                    todo.get_id(),
                    todo.get_title(),
                    todo.get_finished(),
                    todo.get_priority()
                );
            }
            println!("--------------------\n");
        }

        // Anzeigen
        print_list("Lokale Todos", &local);
        print_list("Remote Todos", &remote);

        // 1) Lokale behalten
        println!("Merge Option 1: Lokale Änderungen behalten");
        let merged1 = self.merge_todo_lists(local.clone(), remote.clone());
        print_list("Ergebnis Option 1", &merged1);

        // 2) Remote übernehmen
        println!("Merge Option 2: Remote Änderungen übernehmen");
        let merged2 = remote.clone();
        print_list("Ergebnis Option 2", &merged2);

        // 3) Zusammenführen (merge ohne Duplikate)
        println!("Merge Option 3: Zusammenführen");
        let merged3 = self.merge_todo_lists(local, remote);
        print_list("Ergebnis Option 3", &merged3);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::TodoBuilder;

    fn make_todo(id: u32, title: &str) -> Todo {
        TodoBuilder::new()
            .id(id)
            .title(title)
            .build()
            .expect("Title is required")
    }

    #[test]
    fn test_merge_todo_lists_no_duplicates() {
        let repo = GitRepo::new("/tmp/fakepath"); // Pfad nicht relevant hier


        let todo1 = make_todo(1, "Task 1");
        let todo2 = make_todo(2, "Task 2");
        let todo3 = make_todo(3, "Task 3");

        let mut local = TodoList::new();
        local.add(todo1.clone());
        local.add(todo2.clone());

        let mut remote = TodoList::new();
        remote.add(todo2.clone()); // doppelt
        remote.add(todo3.clone());

        let merged = repo.merge_todo_lists(local, remote);

        // Es sollten genau 3 eindeutige Todos sein
        assert_eq!(merged.todos.len(), 3);

        let titles: Vec<String> = merged.todos.iter().map(|t| t.get_title().clone()).collect();
        assert!(titles.contains(&"Task 1".to_string()));
        assert!(titles.contains(&"Task 2".to_string()));
        assert!(titles.contains(&"Task 3".to_string()));
    }

    #[test]
    fn test_merge_todo_lists_empty_local() {
        let repo = GitRepo::new("/tmp/fakepath");

        let mut local = TodoList::new();

        let mut remote = TodoList::new();
        remote.add(make_todo(1, "Remote Task"));

        let merged = repo.merge_todo_lists(local, remote);

        assert_eq!(merged.todos.len(), 1);
        assert_eq!(merged.todos[0].get_title(), "Remote Task");
    }

    #[test]
    fn test_merge_todo_lists_empty_remote() {
        let repo = GitRepo::new("/tmp/fakepath");

        let mut local = TodoList::new();
        local.add(make_todo(1, "Local Task"));

        let remote = TodoList::new();

        let merged = repo.merge_todo_lists(local, remote);

        assert_eq!(merged.todos.len(), 1);
        assert_eq!(merged.todos[0].get_title(), "Local Task");
    }
}
