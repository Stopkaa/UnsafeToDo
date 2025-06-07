// src/git_sync.rs - Separates Git-Sync Modul

use git2::{Repository, Signature, ObjectType, Oid};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

// GIT CONFIGURATION
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitConfig {
    pub repo_path: PathBuf,
    pub remote_url: Option<String>,
    pub branch: String,
    pub user_name: String,
    pub user_email: String,
}

impl GitConfig {
    fn config_path() -> Result<PathBuf, Box<dyn Error>> {
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        Ok(home.join(".config").join("mytodo").join("git-config.json"))
    }
    
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: GitConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            // Default config
            let home = dirs::home_dir().ok_or("Could not find home directory")?;
            Ok(GitConfig {
                repo_path: home.join(".mytodo"),
                remote_url: None,
                branch: "main".to_string(),
                user_name: "MyTodo User".to_string(),
                user_email: "user@mytodo.local".to_string(),
            })
        }
    }
    
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, json)?;
        Ok(())
    }
    
    pub fn get_todos_path(&self) -> PathBuf {
        self.repo_path.join("todos.json")
    }
    
    pub fn set_remote(&mut self, remote_url: Option<String>) -> Result<(), Box<dyn Error>> {
        self.remote_url = remote_url;
        self.save()
    }
    
    pub fn set_user(&mut self, name: String, email: String) -> Result<(), Box<dyn Error>> {
        self.user_name = name;
        self.user_email = email;
        self.save()
    }
}

// GIT SYNC OPERATIONS
pub struct GitSync {
    config: GitConfig,
}

impl GitSync {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = GitConfig::load()?;
        Ok(GitSync { config })
    }
    
    pub fn with_config(config: GitConfig) -> Self {
        GitSync { config }
    }
    
    // REPOSITORY MANAGEMENT
    pub fn init_repo(&mut self, remote_url: Option<String>) -> Result<(), Box<dyn Error>> {
        let repo_path = &self.config.repo_path;
        
        if repo_path.exists() {
            return Err("Repository already exists. Use 'git-config' commands to modify.".into());
        }
        
        // Create directory
        fs::create_dir_all(repo_path)?;
        
        // Initialize git repo
        let repo = Repository::init(repo_path)?;
        
        // Add remote if provided
        if let Some(ref url) = remote_url {
            repo.remote("origin", url)?;
            println!("✅ Added remote: {}", url);
        }
        
        // Create initial todos.json if it doesn't exist
        let todos_path = self.config.get_todos_path();
        if !todos_path.exists() {
            let initial_todos = crate::todo::TodoList::new();
            let json = serde_json::to_string_pretty(&initial_todos)?;
            fs::write(&todos_path, json)?;
        }
        
        // Create .gitignore
        let gitignore_content = "# MyTodo Git repository\n*.tmp\n*.bak\n.DS_Store\n";
        fs::write(repo_path.join(".gitignore"), gitignore_content)?;
        
        // Initial commit
        self.commit_changes("Initial commit: Setup MyTodo repository")?;
        
        // Update config
        self.config.remote_url = remote_url;
        self.config.save()?;
        
        println!("✅ Git repository initialized at: {}", repo_path.display());
        Ok(())
    }
    
    pub fn clone_repo(&mut self, remote_url: String, local_path: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
        let repo_path = local_path.unwrap_or_else(|| self.config.repo_path.clone());
        
        if repo_path.exists() {
            return Err(format!("Directory already exists: {}", repo_path.display()).into());
        }
        
        println!("📥 Cloning repository from: {}", remote_url);
        let _repo = Repository::clone(&remote_url, &repo_path)?;
        
        // Update config
        self.config.repo_path = repo_path.clone();
        self.config.remote_url = Some(remote_url);
        self.config.save()?;
        
        println!("✅ Repository cloned to: {}", repo_path.display());
        Ok(())
    }
    
    // SYNC OPERATIONS
    pub fn sync(&self) -> Result<(), Box<dyn Error>> {
        println!("🔄 Starting sync...");
        
        // 1. Commit local changes if any
        if self.has_local_changes()? {
            println!("📝 Committing local changes...");
            self.commit_changes("Auto-commit: Update todos")?;
        } else {
            println!("📋 No local changes to commit");
        }
        
        // 2. Pull remote changes if remote exists
        if self.config.remote_url.is_some() {
            println!("⬇️  Pulling remote changes...");
            self.pull_changes()?;
        } else {
            println!("🏠 No remote configured - local only");
        }
        
        // 3. Push local changes if remote exists
        if self.config.remote_url.is_some() {
            println!("⬆️  Pushing local changes...");
            self.push_changes()?;
        }
        
        println!("✅ Sync completed successfully!");
        Ok(())
    }
    
    pub fn pull(&self) -> Result<(), Box<dyn Error>> {
        if self.config.remote_url.is_none() {
            return Err("No remote configured. Use 'git-config set-remote <url>' first.".into());
        }
        
        println!("⬇️  Pulling changes from remote...");
        self.pull_changes()
    }
    
    pub fn push(&self) -> Result<(), Box<dyn Error>> {
        if self.config.remote_url.is_none() {
            return Err("No remote configured. Use 'git-config set-remote <url>' first.".into());
        }
        
        // Commit changes first if any
        if self.has_local_changes()? {
            self.commit_changes("Auto-commit before push")?;
        }
        
        println!("⬆️  Pushing changes to remote...");
        self.push_changes()
    }
    
    pub fn commit(&self, message: Option<String>) -> Result<(), Box<dyn Error>> {
        let commit_message = message.unwrap_or_else(|| "Manual commit: Update todos".to_string());
        
        if !self.has_local_changes()? {
            println!("📋 No changes to commit");
            return Ok(());
        }
        
        self.commit_changes(&commit_message)?;
        println!("✅ Changes committed: {}", commit_message);
        Ok(())
    }
    
    // INTERNAL HELPER METHODS
    fn get_repo(&self) -> Result<Repository, Box<dyn Error>> {
        Repository::open(&self.config.repo_path)
            .map_err(|_| "Git repository not found. Run 'mytodo git init' first.".into())
    }
    
    fn has_local_changes(&self) -> Result<bool, Box<dyn Error>> {
        let repo = self.get_repo()?;
        let statuses = repo.statuses(None)?;
        Ok(!statuses.is_empty())
    }
    
    fn commit_changes(&self, message: &str) -> Result<(), Box<dyn Error>> {
        let repo = self.get_repo()?;
        let mut index = repo.index()?;
        
        // Add all files in repo (todos.json, .gitignore, etc.)
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        
        // Create commit
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        
        let signature = Signature::now(&self.config.user_name, &self.config.user_email)?;
        
        // Get parent commit (if any)
        let parent_commit = match repo.head() {
            Ok(head) => {
                let oid = head.target().ok_or("Invalid HEAD")?;
                Some(repo.find_commit(oid)?)
            }
            Err(_) => None, // First commit
        };
        
        let parents: Vec<&git2::Commit> = match &parent_commit {
            Some(commit) => vec![commit],
            None => vec![],
        };
        
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;
        
        Ok(())
    }
    
    fn pull_changes(&self) -> Result<(), Box<dyn Error>> {
        let repo = self.get_repo()?;
        
        // Fetch from remote
        let mut remote = repo.find_remote("origin")?;
        let mut callbacks = git2::RemoteCallbacks::new();
        
        // Simple auth (you might want to add SSH key support)
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
        });
        
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        
        remote.fetch(&[&self.config.branch], Some(&mut fetch_options), None)?;
        
        // Get remote branch
        let remote_branch_name = format!("refs/remotes/origin/{}", self.config.branch);
        let remote_oid = repo.refname_to_id(&remote_branch_name)?;
        let remote_commit = repo.find_commit(remote_oid)?;
        
        // Get current HEAD
        let head_oid = match repo.head() {
            Ok(head) => head.target().ok_or("Invalid HEAD")?,
            Err(_) => {
                // No HEAD yet (empty repo) - just set to remote
                let refname = format!("refs/heads/{}", self.config.branch);
                repo.reference(&refname, remote_oid, false, "Initial pull")?;
                repo.set_head(&refname)?;
                repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
                println!("✅ Initial pull completed");
                return Ok(());
            }
        };
        
        let head_commit = repo.find_commit(head_oid)?;
        
        // Check if we need to merge
        if remote_oid == head_oid {
            println!("📦 Already up to date");
            return Ok(());
        }
        
        // Simple fast-forward merge
        let (analysis, _) = repo.merge_analysis(&[&remote_commit])?;
        
        if analysis.is_fast_forward() {
            // Fast-forward merge
            let refname = format!("refs/heads/{}", self.config.branch);
            let mut reference = repo.find_reference(&refname)?;
            reference.set_target(remote_oid, "Fast-forward merge")?;
            repo.set_head(&refname)?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            println!("✅ Fast-forward merge completed");
        } else {
            return Err("Merge conflicts detected. Please resolve manually using 'git' commands.".into());
        }
        
        Ok(())
    }
    
    fn push_changes(&self) -> Result<(), Box<dyn Error>> {
        let repo = self.get_repo()?;
        let mut remote = repo.find_remote("origin")?;
        
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
        });
        
        let mut push_options = git2::PushOptions::new();
        push_options.remote_callbacks(callbacks);
        
        let refspec = format!("+refs/heads/{}:refs/heads/{}", self.config.branch, self.config.branch);
        remote.push(&[&refspec], Some(&mut push_options))?;
        
        println!("✅ Changes pushed to remote");
        Ok(())
    }
    
    // STATUS AND INFO
    pub fn status(&self) -> Result<(), Box<dyn Error>> {
        println!("📊 Git Repository Status:");
        println!("   Path: {}", self.config.repo_path.display());
        println!("   Branch: {}", self.config.branch);
        println!("   User: {} <{}>", self.config.user_name, self.config.user_email);
        
        if let Some(remote_url) = &self.config.remote_url {
            println!("   Remote: {}", remote_url);
        } else {
            println!("   Remote: None (local only)");
        }
        
        // Check if repo exists
        match self.get_repo() {
            Ok(repo) => {
                // Check for local changes
                let statuses = repo.statuses(None)?;
                if statuses.is_empty() {
                    println!("   Status: ✅ Clean (no local changes)");
                } else {
                    println!("   Status: 📝 {} uncommitted changes", statuses.len());
                    for entry in statuses.iter() {
                        let path = entry.path().unwrap_or("unknown");
                        let status = match entry.status() {
                            s if s.contains(git2::Status::WT_NEW) => "new",
                            s if s.contains(git2::Status::WT_MODIFIED) => "modified",
                            s if s.contains(git2::Status::WT_DELETED) => "deleted",
                            _ => "unknown",
                        };
                        println!("     - {} ({})", path, status);
                    }
                }
                
                // Show recent commits
                self.show_recent_commits(&repo)?;
            }
            Err(_) => {
                println!("   Status: ❌ Repository not initialized");
            }
        }
        
        Ok(())
    }
    
    fn show_recent_commits(&self, repo: &Repository) -> Result<(), Box<dyn Error>> {
        println!("\n📋 Recent commits:");
        
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        
        for (i, oid) in revwalk.enumerate() {
            if i >= 5 { break; } // Show last 5 commits
            
            let oid = oid?;
            let commit = repo.find_commit(oid)?;
            let short_id = format!("{:.7}", oid);
            let message = commit.message().unwrap_or("").lines().next().unwrap_or("");
            let time = chrono::DateTime::from_timestamp(commit.time().seconds(), 0)
                .unwrap_or_default()
                .format("%Y-%m-%d %H:%M");
            
            println!("   {} {} ({})", short_id, message, time);
        }
        
        Ok(())
    }
    
    // CONFIGURATION METHODS
    pub fn get_config(&self) -> &GitConfig {
        &self.config
    }
    
    pub fn set_remote(&mut self, remote_url: Option<String>) -> Result<(), Box<dyn Error>> {
        // Update git remote if repo exists
        if let Ok(repo) = self.get_repo() {
            match &remote_url {
                Some(url) => {
                    // Try to find existing remote
                    match repo.find_remote("origin") {
                        Ok(mut remote) => {
                            // Update existing remote
                            repo.remote_set_url("origin", url)?;
                            println!("✅ Updated remote URL: {}", url);
                        }
                        Err(_) => {
                            // Add new remote
                            repo.remote("origin", url)?;
                            println!("✅ Added remote: {}", url);
                        }
                    }
                }
                None => {
                    // Remove remote
                    if repo.find_remote("origin").is_ok() {
                        repo.remote_delete("origin")?;
                        println!("✅ Removed remote");
                    }
                }
            }
        }
        
        // Update config
        self.config.set_remote(remote_url)
    }
    
    pub fn set_user(&mut self, name: String, email: String) -> Result<(), Box<dyn Error>> {
        self.config.set_user(name, email)
    }
}

// UTILITY FUNCTIONS
pub fn is_git_repo_initialized() -> bool {
    GitConfig::load()
        .map(|config| config.repo_path.join(".git").exists())
        .unwrap_or(false)
}

pub fn get_git_status_summary() -> Result<String, Box<dyn Error>> {
    let git_sync = GitSync::new()?;
    
    if let Ok(repo) = git_sync.get_repo() {
        let statuses = repo.statuses(None)?;
        if statuses.is_empty() {
            Ok("✅ Clean".to_string())
        } else {
            Ok(format!("📝 {} changes", statuses.len()))
        }
    } else {
        Ok("❌ Not initialized".to_string())
    }
}
