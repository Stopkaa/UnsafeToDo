use std::fs;
use std::io::{self};
use std::path::PathBuf;
use std::process::{Command, Output};

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

        if !output.status.success() {
            eprintln!("Git error: git {}", args.join(" "));
            eprintln!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }

    pub fn pull(&self) -> io::Result<()> {
        println!("Pulling changes from origin...");
        self.run_git_command(&["pull", "origin", "main"])
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
}
