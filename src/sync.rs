use std::fs;
use std::io::{self};
use std::path::PathBuf;
use std::process::{Command, Output};
use crate::display::{display_todo_vector};
use crate::{config, todo};

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

    pub fn handle_merge_conflict(&self) -> io::Result<()> {
        let path = self.path.join(config::TODO_FILE_NAME);
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
                display_todo_vector(&local_todos);

                println!("Incoming version:");
                display_todo_vector(&incoming_todos);

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

    fn ensure_tracking_branch(&self) -> io::Result<()> {
        let branch = self.get_current_branch()?;

        let tracking_status = Command::new("git")
            .args(&["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])
            .current_dir(&self.path)
            .output();

        // Wenn das Kommando fehlschlägt oder nicht 0 zurückgibt
        let needs_tracking = match &tracking_status {
            Err(_) => true,
            Ok(output) if !output.status.success() => true,
            _ => false,
        };

        if needs_tracking {
            // Tracking fehlt → setze Upstream auf origin/main
            let remote_refs = Command::new("git")
                .args(&["ls-remote", "--heads", "origin"])
                .current_dir(&self.path)
                .output()?;

            let output = String::from_utf8_lossy(&remote_refs.stdout);
            let remote_branch = if output.contains("refs/heads/main") {
                "main"
            } else if output.contains("refs/heads/master") {
                "master"
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, "No known default branch found"));
            };

            let set_upstream = Command::new("git")
                .args(&[
                    "branch",
                    "--set-upstream-to",
                    &format!("origin/{}", remote_branch),
                    &branch,
                ])
                .current_dir(&self.path)
                .status()?;

            if !set_upstream.success() {
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to set upstream tracking"));
            } else {
                println!("Tracking set: {} → origin/{}", branch, remote_branch);
            }
        }

        Ok(())
    }


    pub fn pull(&self) -> io::Result<()> {
        println!("Pulling changes from origin...");

        self.ensure_tracking_branch()?; // 🔧 Tracking sicherstellen

        let output = Command::new("git")
            .arg("-C")
            .arg(&self.path)
            .args(&["pull", "--no-rebase", "--allow-unrelated-histories"])
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
                eprintln!("Merge-Conflict detected.");
                self.handle_merge_conflict()?;
                Ok(())
            } else {
                eprintln!("Git pull Error:\n{}", stderr);
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

    fn get_current_branch(&self) -> io::Result<String> {
        let output = Command::new("git")
            .args(&["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(&self.path)
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get current branch",
            ));
        }

        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(branch)
    }

    pub fn push(&self) -> io::Result<()> {
        println!("Pushing changes to origin...");

        let branch = self.get_current_branch()?;
        self.run_git_command(&["push", "-u", "origin", &branch])
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
        let git_dir = self.path.join(".git");

        if git_dir.exists() {
            //println!("Git repository already exists.");

            // Prüfen, ob ein Remote vorhanden ist
            let output = Command::new("git")
                .args(&["remote", "get-url", "origin"])
                .current_dir(&self.path)
                .output()?;

            if !output.status.success() {
                // Kein origin vorhanden, aber remote_url wurde übergeben
                if let Some(remote) = remote_url {
                    println!("Setting remote origin to: {}", remote);
                    let status = Command::new("git")
                        .args(&["remote", "add", "origin", remote])
                        .current_dir(&self.path)
                        .status()?;
                    if !status.success() {
                        eprintln!("Failed to add remote origin.");
                    }
                }
            }

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

            let todos_path = self.path.join(config::TODO_FILE_NAME);
            if !todos_path.exists() {
                fs::write(&todos_path, "")?;
            }

            Command::new("git")
                .args(&["add", config::TODO_FILE_NAME])
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
                } else {
                    // Remote hinzugefügt, jetzt den default branch ermitteln
                    if let Ok(default_branch) = self.get_remote_default_branch() {
                        println!("Remote default branch is '{}'", &default_branch);

                        // Tracking-Branch setzen
                        let branch_output = Command::new("git")
                            .args(&["branch", "--show-current"])
                            .current_dir(&self.path)
                            .output()?;
                        let branch_output_str = String::from_utf8_lossy(&branch_output.stdout);
                        let current_branch = branch_output_str.trim();

                        if current_branch == default_branch {
                            let remote_branch = format!("origin/{}", default_branch);
                            let track_status = Command::new("git")
                                .args(&["branch", "--set-upstream-to", &remote_branch, &default_branch])
                                .current_dir(&self.path)
                                .status()?;

                            if !track_status.success() {
                                eprintln!("Failed to set upstream branch.");
                            } else {
                                println!("Tracking branch set to origin/{}.", default_branch);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn get_remote_default_branch(&self) -> io::Result<String> {
        let output = Command::new("git")
            .args(&["symbolic-ref", "refs/remotes/origin/HEAD"])
            .current_dir(&self.path)
            .output()?;
        if !output.status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to get remote HEAD"));
        }
        let refname = String::from_utf8_lossy(&output.stdout).trim().to_string();
        // refname ist z.B. "refs/remotes/origin/main"
        let branch = refname.rsplit('/').next().unwrap_or("main").to_string();
        Ok(branch)
    }
}
