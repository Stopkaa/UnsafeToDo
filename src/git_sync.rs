use std::error::Error;
use std::path::Path;
use git2::{Repository, Signature, RemoteCallbacks, FetchOptions, PushOptions};

/// Check if a directory is a git repository
pub fn is_git_repository(path: &Path) -> bool {
    path.join(".git").exists()
}

/// Main sync function: add + commit + pull + push
pub fn sync_repository(repo_path: &Path) -> Result<(), Box<dyn Error>> {
    if !is_git_repository(repo_path) {
        return Err(format!("Not a git repository: {}", repo_path.display()).into());
    }
    
    let repo = Repository::open(repo_path)?;
    
    println!("📝 Adding changes...");
    add_todos_file(&repo)?;
    
    println!("💾 Committing changes...");
    let committed = commit_changes(&repo)?;
    if !committed {
        println!("📋 No changes to commit");
    }
    
    // Only pull/push if we have a remote
    if has_remote(&repo)? {
        println!("⬇️ Pulling from remote...");
        pull_from_remote(&repo)?;
        
        println!("⬆️ Pushing to remote...");
        push_to_remote(&repo)?;
    } else {
        println!("🏠 No remote configured - local only");
    }
    
    Ok(())
}

/// Add todos.json to git staging area
pub fn add_todos_file(repo: &Repository) -> Result<(), Box<dyn Error>> {
    let mut index = repo.index()?;
    
    // Add todos.json if it exists
    if repo.workdir().unwrap().join("todos.json").exists() {
        index.add_path(Path::new("todos.json"))?;
    }
    
    // Add .gitignore if it exists
    if repo.workdir().unwrap().join(".gitignore").exists() {
        index.add_path(Path::new(".gitignore"))?;
    }
    
    index.write()?;
    Ok(())
}

/// Commit changes if any exist
pub fn commit_changes(repo: &Repository) -> Result<bool, Box<dyn Error>> {
    let statuses = repo.statuses(None)?;
    
    if statuses.is_empty() {
        return Ok(false); // No changes to commit
    }
    
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    
    let signature = Signature::now("UnsafeToDo", "unsafe.todo@example.com")?;
    
    // Get parent commit if exists
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
        "Update todos",
        &tree,
        &parents,
    )?;
    
    Ok(true) // Changes were committed
}

/// Pull changes from remote repository
pub fn pull_from_remote(repo: &Repository) -> Result<(), Box<dyn Error>> {
    println!("🔍 Checking remote connection...");
    
    let mut remote = repo.find_remote("origin")?;
    
    // Setup SSH authentication with better error handling
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|url, username_from_url, allowed_types| {
        println!("🔑 Authenticating to: {}", url);
        println!("👤 Username: {}", username_from_url.unwrap_or("unknown"));
        
        // Try SSH key from agent first
        if allowed_types.contains(git2::CredentialType::SSH_KEY) {
            match git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                Ok(cred) => {
                    println!("✅ SSH key loaded from agent");
                    return Ok(cred);
                }
                Err(e) => {
                    println!("❌ SSH agent failed: {}", e);
                }
            }
        }
        
        // Fallback: try default SSH key locations
        if allowed_types.contains(git2::CredentialType::SSH_KEY) {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            
            // Try ed25519 key first (modern default)
            let ed25519_private = format!("{}/.ssh/id_ed25519", home);
            let ed25519_public = format!("{}/.ssh/id_ed25519.pub", home);
            
            if std::path::Path::new(&ed25519_private).exists() {
                println!("🔑 Trying ed25519 SSH key: {}", ed25519_private);
                match git2::Cred::ssh_key(
                    username_from_url.unwrap_or("git"),
                    Some(std::path::Path::new(&ed25519_public)),
                    std::path::Path::new(&ed25519_private),
                    None
                ) {
                    Ok(cred) => {
                        println!("✅ ed25519 SSH key loaded");
                        return Ok(cred);
                    }
                    Err(e) => {
                        println!("❌ ed25519 SSH key failed: {}", e);
                    }
                }
            }
            
            // Fallback to RSA key
            let rsa_private = format!("{}/.ssh/id_rsa", home);
            let rsa_public = format!("{}/.ssh/id_rsa.pub", home);
            
            if std::path::Path::new(&rsa_private).exists() {
                println!("🔑 Trying RSA SSH key: {}", rsa_private);
                match git2::Cred::ssh_key(
                    username_from_url.unwrap_or("git"),
                    Some(std::path::Path::new(&rsa_public)),
                    std::path::Path::new(&rsa_private),
                    None
                ) {
                    Ok(cred) => {
                        println!("✅ RSA SSH key loaded");
                        return Ok(cred);
                    }
                    Err(e) => {
                        println!("❌ RSA SSH key failed: {}", e);
                    }
                }
            }
        }
        
        Err(git2::Error::from_str("No authentication method available"))
    });
    
    // Add progress callback to show what's happening
    callbacks.update_tips(|refname, a, b| {
        if a.is_zero() {
            println!("📥 [new]     {:20} {}", b, refname);
        } else {
            println!("📥 [updated] {:10}..{:10} {}", a, b, refname);
        }
        true
    });
    
    callbacks.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            print!("Resolving deltas {}/{}\r", stats.indexed_deltas(), stats.total_deltas());
        } else if stats.total_objects() > 0 {
            print!("Receiving objects: {}% ({}/{})\r", 
                (100 * stats.received_objects()) / stats.total_objects(),
                stats.received_objects(),
                stats.total_objects());
        }
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        true
    });
    
    // Create fetch options with authentication
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    
    // Try to fetch with timeout handling
    println!("📡 Fetching from remote...");
    match remote.fetch(&["main"], Some(&mut fetch_options), None) {
        Ok(_) => println!("✅ Fetch completed"),
        Err(e) => {
            println!("❌ Fetch failed: {}", e);
            return Err(format!("Failed to fetch from remote: {}", e).into());
        }
    }
    
    // Rest of the merge logic
    let remote_oid = match repo.refname_to_id("refs/remotes/origin/main") {
        Ok(oid) => oid,
        Err(_) => {
            println!("❌ No 'main' branch found on remote");
            return Err("Remote branch 'refs/remotes/origin/main' not found".into());
        }
    };
    
    let remote_annotated_commit = repo.find_annotated_commit(remote_oid)?;
    
    // Check if we have local commits
    let head_oid = match repo.head() {
        Ok(head) => head.target().ok_or("Invalid HEAD")?,
        Err(_) => {
            // No local commits yet - first pull
            println!("🆕 No local commits, setting up initial branch...");
            repo.reference("refs/heads/main", remote_oid, false, "Initial pull")?;
            repo.set_head("refs/heads/main")?;
            repo.checkout_head(None)?;
            println!("✅ Initial pull completed");
            return Ok(());
        }
    };
    
    // Check if already up to date
    if remote_oid == head_oid {
        println!("📦 Already up to date");
        return Ok(());
    }
    
    // Analyze merge
    println!("🔍 Analyzing merge...");
    let (analysis, _) = repo.merge_analysis(&[&remote_annotated_commit])?;
    
    if analysis.is_fast_forward() {
        println!("⏩ Fast-forwarding...");
        let mut reference = repo.find_reference("refs/heads/main")?;
        reference.set_target(remote_oid, "Fast-forward merge")?;
        repo.set_head("refs/heads/main")?;
        repo.checkout_head(None)?;
        println!("✅ Fast-forward merge completed");
    } else if analysis.is_up_to_date() {
        println!("📦 Already up to date");
    } else {
        println!("❌ Complex merge required");
        return Err("Cannot fast-forward. Manual merge required.".into());
    }
    
    Ok(())
}

/// Push changes to remote repository
pub fn push_to_remote(repo: &Repository) -> Result<(), Box<dyn Error>> {
    let mut remote = repo.find_remote("origin")?;
    
    // Setup SSH authentication
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
    });
    
    // Create push options with authentication
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);
    
    // Push to remote
    remote.push(&["refs/heads/main:refs/heads/main"], Some(&mut push_options))?;
    println!("✅ Push completed");
    
    Ok(())
}

/// Check if repository has a remote named 'origin'
pub fn has_remote(repo: &Repository) -> Result<bool, Box<dyn Error>> {
    Ok(repo.find_remote("origin").is_ok())
}

/// Get basic repository status
pub fn get_repo_status(repo_path: &Path) -> Result<GitStatus, Box<dyn Error>> {
    if !is_git_repository(repo_path) {
        return Err(format!("Not a git repository: {}", repo_path.display()).into());
    }
    
    let repo = Repository::open(repo_path)?;
    
    // Check for remote
    let has_remote = has_remote(&repo)?;
    let remote_url = if has_remote {
        repo.find_remote("origin")?.url().map(|s| s.to_string())
    } else {
        None
    };
    
    // Check for uncommitted changes
    let statuses = repo.statuses(None)?;
    let has_changes = !statuses.is_empty();
    let change_count = statuses.len();
    
    // Get current branch
    let current_branch = repo.head()
        .ok()
        .and_then(|head| head.shorthand().map(|s| s.to_string()));
    
    Ok(GitStatus {
        has_remote,
        remote_url,
        has_changes,
        current_branch,
        change_count,
    })
}

/// Git repository status information
#[derive(Debug)]
pub struct GitStatus {
    pub has_remote: bool,
    pub remote_url: Option<String>,
    pub has_changes: bool,
    pub current_branch: Option<String>,
    pub change_count: usize,
}

impl GitStatus {
    pub fn display(&self) {
        println!("📊 Git Status:");
        
        if let Some(branch) = &self.current_branch {
            println!("   Branch: {}", branch);
        } else {
            println!("   Branch: No commits yet");
        }
        
        if self.has_remote {
            if let Some(url) = &self.remote_url {
                println!("   Remote: {}", url);
            } else {
                println!("   Remote: origin (no URL)");
            }
        } else {
            println!("   Remote: None (local only)");
        }
        
        if self.has_changes {
            println!("   Status: 📝 {} uncommitted changes", self.change_count);
        } else {
            println!("   Status: ✅ Clean (no changes)");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;
    
    #[test]
    fn test_is_git_repository() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();
        
        // Not a git repo initially
        assert!(!is_git_repository(path));
        
        // Create .git directory
        fs::create_dir_all(path.join(".git")).unwrap();
        assert!(is_git_repository(path));
    }
    
    #[test]
    fn test_sync_non_git_repository() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();
        
        let result = sync_repository(path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Not a git repository"));
    }
    
    #[test]
    fn test_git_status_creation() {
        let status = GitStatus {
            has_remote: false,
            remote_url: None,
            has_changes: false,
            current_branch: Some("main".to_string()),
            change_count: 0,
        };
        
        assert_eq!(status.current_branch, Some("main".to_string()));
        assert!(!status.has_remote);
        assert!(!status.has_changes);
    }
}
