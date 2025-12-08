use crate::error::{BiaoError, Result};
use crate::models::{CreateLabelRequest, GithubLabel, UpdateLabelRequest};
use std::process::Command;

pub struct GithubClient {
    owner: String,
    repo: String,
}

impl GithubClient {
    pub fn new(owner: String, repo: String) -> Self {
        Self { owner, repo }
    }

    pub fn repo_url(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }

    fn run_gh(&self, args: &[&str]) -> Result<String> {
        let mut cmd = Command::new("gh");
        cmd.args(["api"]);
        cmd.args(args);

        let output = cmd.output().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                BiaoError::GhNotFound {
                    message: "github.com/cli/cli".to_string(),
                }
            } else {
                BiaoError::GhError {
                    message: format!("Failed to execute gh: {}", e),
                }
            }
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(BiaoError::GhError { message: stderr });
        }

        Ok(String::from_utf8(output.stdout)
            .map_err(|e| BiaoError::GhError {
                message: format!("Invalid UTF-8 from gh: {}", e),
            })?
            .trim()
            .to_string())
    }

    pub async fn list_labels(&self) -> Result<Vec<GithubLabel>> {
        let path = format!("repos/{}/{}/labels", self.owner, self.repo);
        let output = self.run_gh(&[&path])?;
        
        let labels: Vec<GithubLabel> =
            serde_json::from_str(&output).map_err(|e| BiaoError::ParseError {
                message: format!("Failed to parse labels: {}", e),
            })?;

        Ok(labels)
    }

    pub async fn get_label(&self, name: &str) -> Result<GithubLabel> {
        let path = format!("repos/{}/{}/labels/{}", self.owner, self.repo, name);
        let output = self.run_gh(&[&path])?;
        
        let label: GithubLabel =
            serde_json::from_str(&output).map_err(|e| BiaoError::ParseError {
                message: format!("Failed to parse label: {}", e),
            })?;

        Ok(label)
    }

    pub async fn create_label(&self, label: &CreateLabelRequest) -> Result<GithubLabel> {
        let path = format!("repos/{}/{}/labels", self.owner, self.repo);
        
        let name_arg = format!("name={}", label.name);
        let color_arg = format!("color={}", label.color);
        
        let mut args = vec![
            path.as_str(),
            "-f", &name_arg,
            "-f", &color_arg,
        ];

        let desc_arg;
        if let Some(desc) = &label.description {
            desc_arg = format!("description={}", desc);
            args.push("-f");
            args.push(&desc_arg);
        }

        let output = self.run_gh(&args)?;
        
        let created: GithubLabel =
            serde_json::from_str(&output).map_err(|e| BiaoError::ParseError {
                message: format!("Failed to parse created label: {}", e),
            })?;

        Ok(created)
    }

    pub async fn update_label(
        &self,
        name: &str,
        label: &UpdateLabelRequest,
    ) -> Result<GithubLabel> {
        let path = format!("repos/{}/{}/labels/{}", self.owner, self.repo, name);
        
        let mut args: Vec<&str> = vec![path.as_str(), "-X", "PATCH"];
        let mut arg_storage: Vec<String> = Vec::new();

        if let Some(new_name) = &label.name {
            arg_storage.push(format!("name={}", new_name));
        }

        if let Some(color) = &label.color {
            arg_storage.push(format!("color={}", color));
        }

        if let Some(desc) = &label.description {
            arg_storage.push(format!("description={}", desc));
        }

        for arg in &arg_storage {
            args.push("-f");
            args.push(arg);
        }

        let output = self.run_gh(&args)?;
        
        let updated: GithubLabel =
            serde_json::from_str(&output).map_err(|e| BiaoError::ParseError {
                message: format!("Failed to parse updated label: {}", e),
            })?;

        Ok(updated)
    }

    pub async fn delete_label(&self, name: &str) -> Result<()> {
        let path = format!("repos/{}/{}/labels/{}", self.owner, self.repo, name);
        self.run_gh(&[&path, "-X", "DELETE"])?;
        Ok(())
    }
}
