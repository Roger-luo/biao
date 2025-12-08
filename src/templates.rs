use crate::error::{BiaoError, Result};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Template metadata
#[derive(Debug, Clone)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct TemplateFileMetadata {
    #[serde(default)]
    description: Option<String>,
}

/// Template manager for discovering and loading templates
pub struct TemplateManager {
    template_dirs: Vec<PathBuf>,
}

impl TemplateManager {
    /// Create a new template manager
    pub fn new() -> Result<Self> {
        let mut template_dirs = Vec::new();

        // Add built-in templates (embedded in binary)
        // These are defined as constants
        template_dirs.push(PathBuf::from("__builtin__"));

        // Add user config directory: ~/.config/biao/templates
        if let Ok(home) = std::env::var("HOME") {
            let user_templates = PathBuf::from(home)
                .join(".config/biao/templates");
            if user_templates.exists() {
                template_dirs.push(user_templates);
            }
        }

        // Add installation directory: /usr/local/share/biao/templates (for package managers)
        let install_templates = PathBuf::from("/usr/local/share/biao/templates");
        if install_templates.exists() {
            template_dirs.push(install_templates);
        }

        Ok(TemplateManager { template_dirs })
    }

    /// List all available templates
    pub fn list(&self) -> Result<Vec<TemplateInfo>> {
        use std::collections::HashMap;

        // Use a map to prefer user/system templates over built-ins on name collision
        let mut map: HashMap<String, TemplateInfo> = HashMap::new();

        // 1) User/system dirs (higher priority)
        for dir in &self.template_dirs {
            if dir.as_os_str() == "__builtin__" {
                continue;
            }
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "toml") {
                        if let Some(name) = path.file_stem() {
                            let key = name.to_string_lossy().to_string();
                            let description = Self::read_description_from_file(&path)
                                .unwrap_or_else(|| "User template".to_string());
                            map.entry(key.clone()).or_insert(TemplateInfo {
                                name: key,
                                description,
                                path,
                            });
                        }
                    }
                }
            }
        }

        // 2) Built-ins (lower priority, only if not already present)
        for (name, description) in Self::builtin_templates() {
            let key = name.to_string();
            map.entry(key.clone()).or_insert(TemplateInfo {
                name: key,
                description: description.to_string(),
                path: PathBuf::from("__builtin__"),
            });
        }

        // Collect and sort for stable output
        let mut templates: Vec<TemplateInfo> = map.into_values().collect();
        templates.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(templates)
    }

    /// Get a specific template by name
    pub fn get(&self, name: &str) -> Result<String> {
        // Prefer user/system templates first
        for dir in &self.template_dirs {
            if dir.as_os_str() == "__builtin__" {
                continue;
            }
            let path = dir.join(format!("{}.toml", name));
            if path.exists() {
                return fs::read_to_string(&path).map_err(|e| {
                    BiaoError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to read template '{}': {}", name, e),
                    ))
                });
            }
        }

        // Fallback to built-ins
        if let Some(content) = Self::get_builtin_template(name) {
            return Ok(content);
        }

        Err(BiaoError::InvalidInput(format!(
            "Template '{}' not found. Use 'biao template list' to see available templates.",
            name
        )))
    }

    /// Built-in templates
    fn builtin_templates() -> Vec<(&'static str, &'static str)> {
        vec![
            ("standard", "Standard GitHub labels (bug, feature, documentation, etc.)"),
            ("semantic", "Semantic labels (breaking, feature, bugfix, docs, etc.)"),
            ("priority", "Priority-based labels (critical, high, medium, low)"),
            ("priority-prefixed", "Rust-style priority labels (P-critical, P-high, etc.)"),
            ("type", "Type-based labels (type/bug, type/feature, type/docs, etc.)"),
            ("area", "Area-based labels (area/api, area/cli, area/docs, etc.)"),
            ("operational", "Operational labels (O-hiring, O-roadmap, etc.)"),
        ]
    }

    /// Get built-in template content
    fn get_builtin_template(name: &str) -> Option<String> {
        match name {
            "standard" => Some(TEMPLATE_STANDARD.to_string()),
            "semantic" => Some(TEMPLATE_SEMANTIC.to_string()),
            "priority" => Some(TEMPLATE_PRIORITY.to_string()),
            "priority-prefixed" => Some(TEMPLATE_PRIORITY_PREFIXED.to_string()),
            "type" => Some(TEMPLATE_TYPE.to_string()),
            "area" => Some(TEMPLATE_AREA.to_string()),
            "operational" => Some(TEMPLATE_OPERATIONAL.to_string()),
            _ => None,
        }
    }

    fn read_description_from_file(path: &Path) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        let meta: TemplateFileMetadata = toml::from_str(&content).ok()?;
        meta.description
    }
}

impl Default for TemplateManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| TemplateManager {
            template_dirs: vec![PathBuf::from("__builtin__")],
        })
    }
}

// Built-in template definitions

const TEMPLATE_STANDARD: &str = r#"# Standard GitHub Labels Template
# Common labels used in most GitHub projects

[[labels]]
name = "bug"
color = "d73a49"
description = "Something isn't working"
update_if_match = ["Bug", "bug-report", "bug", "C-bug"]

[[labels]]
name = "feature"
color = "a2eeef"
description = "New feature or request"
update_if_match = ["Feature request", "enhancement", "feature", "C-feature"]

[[labels]]
name = "documentation"
color = "0075ca"
description = "Improvements or additions to documentation"
update_if_match = ["docs", "documentation", "C-documentation"]

[[labels]]
name = "good first issue"
color = "7057ff"
description = "Good for newcomers"
update_if_match = ["good-first-issue", "good first issue", "C-good-first-issue"]

[[labels]]
name = "help wanted"
color = "008672"
description = "Extra attention is needed"
update_if_match = ["help-wanted", "needs-help", "help wanted", "C-help-wanted"]

[[labels]]
name = "invalid"
color = "e4e669"
description = "This doesn't seem right"
update_if_match = ["invalid", "C-invalid"]

[[labels]]
name = "question"
color = "d876e3"
description = "Further information is requested"
update_if_match = ["question", "C-question"]

[[labels]]
name = "wontfix"
color = "ffffff"
description = "This will not be worked on"
update_if_match = ["wontfix", "won't fix", "C-wontfix"]

delete = ["duplicate", "type/bug", "type/feature"]
"#;

const TEMPLATE_SEMANTIC: &str = r#"# Semantic Release Labels Template
# Labels following semantic versioning conventions

[[labels]]
name = "breaking"
color = "d73a49"
description = "Breaking change - requires major version bump"
update_if_match = ["S-breaking"]

[[labels]]
name = "feature"
color = "a2eeef"
description = "New feature - requires minor version bump"
update_if_match = ["S-feature"]

[[labels]]
name = "bugfix"
color = "fbca04"
description = "Bug fix - requires patch version bump"
update_if_match = ["S-bugfix"]

[[labels]]
name = "docs"
color = "0075ca"
description = "Documentation updates"
update_if_match = ["S-docs"]

[[labels]]
name = "refactor"
color = "7057ff"
description = "Code refactoring without feature changes"
update_if_match = ["S-refactor"]

[[labels]]
name = "test"
color = "008672"
description = "Test additions or improvements"
update_if_match = ["S-test"]

[[labels]]
name = "chore"
color = "cccccc"
description = "Maintenance or tool updates"
update_if_match = ["S-chore"]

[[labels]]
name = "ci"
color = "e4e669"
description = "CI/CD and automation changes"
update_if_match = ["S-ci"]
"#;

const TEMPLATE_PRIORITY: &str = r#"# Priority-Based Labels Template
# Use priority levels to triage and prioritize work

[[labels]]
name = "priority/critical"
color = "b60205"
description = "Critical priority - must be addressed immediately"
update_if_match = ["P0", "critical", "P-critical", "priority/critical", "Critical"]

[[labels]]
name = "priority/high"
color = "d73a49"
description = "High priority - address soon"
update_if_match = ["P1", "urgent", "P-high", "priority/high", "High"]

[[labels]]
name = "priority/medium"
color = "f0883e"
description = "Medium priority - address when available"
update_if_match = ["P2", "P-medium", "priority/medium", "Medium"]

[[labels]]
name = "priority/low"
color = "0075ca"
description = "Low priority - address eventually"
update_if_match = ["P3", "nice-to-have", "P-low", "priority/low", "Low"]

[[labels]]
name = "priority/backlog"
color = "cccccc"
description = "Backlog - not currently planned"
update_if_match = ["P4", "P-backlog", "priority/backlog", "Backlog"]
"#;

const TEMPLATE_PRIORITY_PREFIXED: &str = r#"# Priority Labels Template (P- prefixes)
# Rust-style priority labels

[[labels]]
name = "P-critical"
color = "b60205"
description = "Priority: Critical - must be addressed immediately"
update_if_match = ["priority/critical", "critical", "P0", "Critical"]

[[labels]]
name = "P-high"
color = "d73a49"
description = "Priority: High - address soon"
update_if_match = ["priority/high", "urgent", "P1", "High"]

[[labels]]
name = "P-medium"
color = "f0883e"
description = "Priority: Medium - address when available"
update_if_match = ["priority/medium", "P2", "Medium"]

[[labels]]
name = "P-low"
color = "0075ca"
description = "Priority: Low - address eventually"
update_if_match = ["priority/low", "nice-to-have", "P3", "Low"]

[[labels]]
name = "P-backlog"
color = "cccccc"
description = "Priority: Backlog - not currently planned"
update_if_match = ["priority/backlog", "P4", "Backlog"]
"#;

const TEMPLATE_TYPE: &str = r#"# Type-Based Labels Template
# Categorize issues and PRs by type

[[labels]]
name = "type/bug"
color = "d73a49"
description = "Bug report"
update_if_match = ["bug", "Bug", "type/bug", "T-bug"]

[[labels]]
name = "type/feature"
color = "a2eeef"
description = "Feature request"
update_if_match = ["feature", "Feature", "type/feature", "T-feature"]

[[labels]]
name = "type/enhancement"
color = "a2eeef"
description = "Enhancement to existing feature"
update_if_match = ["enhancement", "type/enhancement", "Enhancement", "T-enhancement"]

[[labels]]
name = "type/docs"
color = "0075ca"
description = "Documentation"
update_if_match = ["docs", "documentation", "type/docs", "T-docs"]

[[labels]]
name = "type/question"
color = "d876e3"
description = "Question or discussion"
update_if_match = ["question", "discussion", "type/question", "Question", "T-question"]

[[labels]]
name = "type/test"
color = "008672"
description = "Test improvements"
update_if_match = ["type/test", "tests", "T-test"]

[[labels]]
name = "type/refactor"
color = "7057ff"
description = "Refactoring"
update_if_match = ["type/refactor", "refactor", "T-refactor"]

[[labels]]
name = "type/chore"
color = "cccccc"
description = "Chores and maintenance"
update_if_match = ["chore", "type/chore", "Chore", "T-chore"]
"#;

const TEMPLATE_AREA: &str = r#"# Area-Based Labels Template (Rust-style prefixes)
# Organize issues and PRs by area of the codebase

[[labels]]
name = "A-api"
color = "0075ca"
description = "Area: API"

[[labels]]
name = "A-cli"
color = "0075ca"
description = "Area: CLI"

[[labels]]
name = "A-docs"
color = "0075ca"
description = "Area: Documentation"

[[labels]]
name = "A-core"
color = "0075ca"
description = "Area: Core"

[[labels]]
name = "A-testing"
color = "0075ca"
description = "Area: Testing"

[[labels]]
name = "A-ci"
color = "0075ca"
description = "Area: CI/CD"

[[labels]]
name = "A-performance"
color = "fbca04"
description = "Area: Performance"

[[labels]]
name = "A-security"
color = "d73a49"
description = "Area: Security"
"#;

const TEMPLATE_OPERATIONAL: &str = r#"# Operational Labels Template (O- prefixes)
# Operational workstreams and meta items

[[labels]]
name = "O-hiring"
color = "0075ca"
description = "Operational: Hiring and staffing"

[[labels]]
name = "O-roadmap"
color = "a2eeef"
description = "Operational: Roadmap and planning"

[[labels]]
name = "O-incident"
color = "d73a49"
description = "Operational: Incident response and follow-up"

[[labels]]
name = "O-maintenance"
color = "cccccc"
description = "Operational: Maintenance and chores"

[[labels]]
name = "O-compliance"
color = "e4e669"
description = "Operational: Compliance, audits, and risk"
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_manager_creation() {
        let manager = TemplateManager::default();
        let templates = manager.list().unwrap();
        assert!(!templates.is_empty());
        assert!(templates.iter().any(|t| t.name == "standard"));
    }

    #[test]
    fn test_get_builtin_template() {
        let content = TemplateManager::get_builtin_template("standard");
        assert!(content.is_some());
        let content = content.unwrap();
        assert!(content.contains("bug"));
        assert!(content.contains("feature"));
    }

    #[test]
    fn test_get_template() {
        let manager = TemplateManager::default();
        let content = manager.get("standard").unwrap();
        assert!(content.contains("bug"));
        assert!(content.contains("feature"));
    }

    #[test]
    fn test_template_not_found() {
        let manager = TemplateManager::default();
        let result = manager.get("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_template_list_reads_description() {
        let temp_dir = std::env::temp_dir()
            .join(format!("biao_template_desc_{}", std::process::id()));
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).ok();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let template_path = temp_dir.join("custom.toml");
        let content = r#"description = "Custom description"

[[labels]]
name = "x"
color = "000000"
"#;
        fs::write(&template_path, content).unwrap();

        let manager = TemplateManager {
            template_dirs: vec![temp_dir.clone(), PathBuf::from("__builtin__")],
        };
        let templates = manager.list().unwrap();
        let custom = templates
            .iter()
            .find(|t| t.name == "custom")
            .expect("custom template not found");
        assert_eq!(custom.description, "Custom description");

        fs::remove_dir_all(&temp_dir).ok();
    }
}
