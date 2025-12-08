use crate::error::{BiaoError, Result};
use std::path::PathBuf;
use std::process::Command;

/// Find the root of the git repository by searching up from current directory
pub fn find_git_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                BiaoError::InvalidInput("git command not found. Please install git.".to_string())
            } else {
                BiaoError::Io(e)
            }
        })?;

    if !output.status.success() {
        return Err(BiaoError::InvalidInput(
            "Not a git repository. Run this command from within a git repository.".to_string(),
        ));
    }

    let path = String::from_utf8(output.stdout)
        .map_err(|_| {
            BiaoError::InvalidInput("Failed to parse git root path".to_string())
        })?
        .trim()
        .to_string();

    Ok(PathBuf::from(path))
}

/// Extract owner and repo from git remote URL
/// Supports:
/// - https://github.com/owner/repo.git
/// - git@github.com:owner/repo.git
/// - https://github.com/owner/repo
/// - git@github.com:owner/repo
pub fn get_repo_info() -> Result<(String, String)> {
    let output = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output()
        .map_err(|e| BiaoError::Io(e))?;

    if !output.status.success() {
        return Err(BiaoError::InvalidInput(
            "Could not find remote.origin.url. Make sure your repository has an origin remote pointing to GitHub.".to_string(),
        ));
    }

    let url = String::from_utf8(output.stdout)
        .map_err(|_| BiaoError::InvalidInput("Failed to parse git remote URL".to_string()))?
        .trim()
        .to_string();

    parse_github_url(&url)
}

fn parse_github_url(url: &str) -> Result<(String, String)> {
    // Handle https://github.com/owner/repo.git
    if let Some(path) = url.strip_prefix("https://github.com/") {
        return extract_owner_repo(path);
    }

    // Handle git@github.com:owner/repo.git
    if let Some(path) = url.strip_prefix("git@github.com:") {
        return extract_owner_repo(path);
    }

    Err(BiaoError::InvalidInput(
        format!(
            "Unsupported remote URL. Only GitHub HTTPS and SSH URLs are supported.\nRemote URL: {}",
            url
        ),
    ))
}

fn extract_owner_repo(path: &str) -> Result<(String, String)> {
    // Remove .git suffix if present
    let path = path.strip_suffix(".git").unwrap_or(path);

    let parts: Vec<&str> = path.split('/').collect();

    if parts.len() < 2 {
        return Err(BiaoError::InvalidInput(
            "Could not parse owner and repo from remote URL".to_string(),
        ));
    }

    Ok((parts[0].to_string(), parts[1].to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_https_url() {
        let (owner, repo) = parse_github_url("https://github.com/cli/cli.git").unwrap();
        assert_eq!(owner, "cli");
        assert_eq!(repo, "cli");
    }

    #[test]
    fn test_parse_https_url_no_git() {
        let (owner, repo) = parse_github_url("https://github.com/cli/cli").unwrap();
        assert_eq!(owner, "cli");
        assert_eq!(repo, "cli");
    }

    #[test]
    fn test_parse_ssh_url() {
        let (owner, repo) = parse_github_url("git@github.com:cli/cli.git").unwrap();
        assert_eq!(owner, "cli");
        assert_eq!(repo, "cli");
    }

    #[test]
    fn test_parse_ssh_url_no_git() {
        let (owner, repo) = parse_github_url("git@github.com:cli/cli").unwrap();
        assert_eq!(owner, "cli");
        assert_eq!(repo, "cli");
    }
}
