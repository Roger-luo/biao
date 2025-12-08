use crate::error::{BiaoError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct LabelConfig {
    #[serde(default)]
    pub labels: Vec<Label>,
    #[serde(default)]
    pub delete: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Label {
    pub name: String,
    /// Color is required for new labels, optional for updates
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    /// List of existing label names to update/rename to this label's name
    #[serde(default)]
    pub update_if_match: Vec<String>,
    /// If true, skip if label already exists. If false (default), fail on existing labels.
    #[serde(default)]
    pub skip_if_exists: bool,
    /// If true, update the label if it already exists instead of failing/skipping.
    #[serde(default)]
    pub update_if_exists: bool,
}

impl LabelConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| {
            BiaoError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read config file: {}", e),
            ))
        })?;

        toml::from_str(&content).map_err(|e| {
            BiaoError::InvalidInput(format!("Failed to parse TOML config: {}", e))
        })
    }

    pub fn has_actions(&self) -> bool {
        !self.labels.is_empty() || !self.delete.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_labels() {
        let toml = r#"
[[labels]]
name = "bug"
color = "d73a49"
description = "Something isn't working"

[[labels]]
name = "feature"
color = "a2eeef"
"#;

        let config: LabelConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.labels.len(), 2);
        assert_eq!(config.labels[0].name, "bug");
        assert_eq!(config.labels[0].color, Some("d73a49".to_string()));
        assert_eq!(config.labels[1].name, "feature");
    }

    #[test]
    fn test_parse_update_labels() {
        let toml = r#"
[[labels]]
name = "bug"
update_if_match = ["bug-report"]
color = "ff0000"

[[labels]]
name = "feature"
description = "New feature request"
"#;

        let config: LabelConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.labels.len(), 2);
        assert_eq!(config.labels[0].name, "bug");
        assert_eq!(config.labels[0].update_if_match, vec!["bug-report".to_string()]);
        assert_eq!(config.labels[1].description, Some("New feature request".to_string()));
    }

    #[test]
    fn test_parse_delete_labels() {
        let toml = r#"
delete = ["wontfix", "invalid"]
"#;

        let config: LabelConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.delete.len(), 2);
        assert_eq!(config.delete[0], "wontfix");
    }

    #[test]
    fn test_parse_mixed() {
        let toml = r#"
delete = ["duplicate"]

[[labels]]
name = "priority-high"
color = "d73a49"

[[labels]]
name = "bug"
description = "Updated description"
"#;

        let config: LabelConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.labels.len(), 2);
        assert_eq!(config.delete.len(), 1);
    }

    #[test]
    fn test_parse_with_conflict_flags() {
        let toml = r#"
[[labels]]
name = "bug"
color = "d73a49"
skip_if_exists = true

[[labels]]
name = "feature"
color = "a2eeef"
update_if_exists = true

[[labels]]
name = "enhancement"
color = "84b6eb"
"#;

        let config: LabelConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.labels.len(), 3);
        assert_eq!(config.labels[0].skip_if_exists, true);
        assert_eq!(config.labels[0].update_if_exists, false);
        assert_eq!(config.labels[1].skip_if_exists, false);
        assert_eq!(config.labels[1].update_if_exists, true);
        assert_eq!(config.labels[2].skip_if_exists, false);
        assert_eq!(config.labels[2].update_if_exists, false);
    }

    #[test]
    fn test_update_if_match() {
        let toml = r#"
[[labels]]
name = "needs-help"
update_if_match = ["help wanted", "help-needed"]
color = "008672"
description = "Extra attention needed"
"#;

        let config: LabelConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.labels.len(), 1);
        assert_eq!(config.labels[0].name, "needs-help");
        assert_eq!(config.labels[0].update_if_match.len(), 2);
        assert_eq!(config.labels[0].update_if_match[0], "help wanted");
        assert_eq!(config.labels[0].update_if_match[1], "help-needed");
    }
}
