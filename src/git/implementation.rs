use std::path::Path;
use std::process::Command;

use super::error::{self, GitError};
use super::models::Commit;
use super::repository::GitRepository;

const FORMAT: &str = r#"{"author": "%ae", "time": %at, "message": "%s"}"#;

pub struct GitCli;

impl GitCli {
    fn ensure_git() -> error::Result<'static, ()> {
        let check_command_output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", "where git"]).output()?
        } else {
            Command::new("sh").args(["-c", "which git"]).output()?
        };
        let has_installed_git = check_command_output.status.success();

        if !has_installed_git {
            return Err(GitError::MissingGitCli);
        }

        Ok(())
    }

    fn ensure_repository(path: &Path) -> error::Result<()> {
        let check_repository_output = Command::new("git").arg("status").output()?;
        let exist_a_repository = check_repository_output.status.success();

        if !exist_a_repository {
            return Err(GitError::NoRepository(path));
        }

        Ok(())
    }

    fn translate_commits(git_log: &str) -> Vec<Commit> {
        let mut commits = Vec::new();

        for line in git_log.lines() {
            if let Ok(commit) = serde_json::from_str::<Commit>(line) {
                commits.push(commit);
            } else if let Some(Commit { changes, .. }) = commits.last_mut() {
                if line.contains("diff") {
                    changes.push(line.to_string())
                } else if let Some(change) = changes.last_mut() {
                    change.push('\n');
                    change.push_str(line);
                }
            }
        }

        commits
    }
}

impl GitRepository for GitCli {
    fn get_all_commits<'a>(&self, path: &'a Path) -> error::Result<'a, Vec<Commit>> {
        GitCli::ensure_git()?;
        GitCli::ensure_repository(path)?;

        let git_log_output = Command::new("git")
            .args(["log", "-p", "--no-merges"])
            .arg(format!("--format={FORMAT}"))
            .current_dir(path)
            .output()?;

        if !git_log_output.status.success() {
            let git_log_error = String::from_utf8_lossy(&git_log_output.stderr);

            let error_msg = if git_log_error.is_empty() {
                "Failure when trying to obtain commits"
            } else {
                &git_log_error
            };

            return Err(GitError::Other(error_msg.to_string()));
        }

        let git_log = String::from_utf8_lossy(&git_log_output.stdout);

        Ok(Self::translate_commits(&git_log))
    }
}
