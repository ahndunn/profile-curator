use std::path::{Path, PathBuf};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::patch::ProfilePatch;

/// Metadata record for a profile modification event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChangelogEntry {
    /// Timestamp of when the patch was applied in UTC ISO 8601.
    pub timestamp: String,
    /// Human- or agent-provided rationale or intent for the changes.
    pub message: String,
    /// List of human-readable differences applied during this change.
    pub changes: Vec<String>,
    /// The patch payload that produced these changes.
    pub patch: ProfilePatch,
}

/// Changelog manager responsible for saving dual-format (.json and .md) changelog entries.
pub struct ChangelogManager {
    dir: PathBuf,
}

impl ChangelogManager {
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        Self {
            dir: dir.as_ref().to_path_buf(),
        }
    }

    /// Record a patch application event into the changelog directory.
    /// Creates both:
    /// 1. `<TIMESTAMP>_<SLUG>.json` (Full structured patch and metadata)
    /// 2. `<TIMESTAMP>_<SLUG>.md` (Scannable Markdown narrative log)
    pub fn record_change(
        &self,
        message: &str,
        changes: &[String],
        patch: &ProfilePatch,
    ) -> std::io::Result<(PathBuf, PathBuf)> {
        std::fs::create_dir_all(&self.dir)?;

        let now = Utc::now();
        let timestamp_iso = now.to_rfc3339();
        let timestamp_file = now.format("%Y%m%dT%H%M%SZ").to_string();

        let slug = sanitize_slug(message);
        let base_name = if slug.is_empty() {
            format!("{}_patch", timestamp_file)
        } else {
            format!("{}_{}", timestamp_file, slug)
        };

        let json_path = self.dir.join(format!("{}.json", base_name));
        let md_path = self.dir.join(format!("{}.md", base_name));

        let entry = ChangelogEntry {
            timestamp: timestamp_iso.clone(),
            message: message.to_string(),
            changes: changes.to_vec(),
            patch: patch.clone(),
        };

        // Write .json log
        let json_content = serde_json::to_string_pretty(&entry)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(&json_path, json_content)?;

        // Write .md log
        let mut md_content = "# Profile Change Record\n\n".to_string();
        md_content.push_str(&format!("- **Timestamp**: {}\n", timestamp_iso));
        md_content.push_str(&format!("- **Message**: {}\n\n", message));
        md_content.push_str("## Applied Diffs\n\n");
        if changes.is_empty() {
            md_content.push_str("_No state changes detected (idempotent patch)._\n");
        } else {
            for ch in changes {
                md_content.push_str(&format!("- {}\n", ch));
            }
        }
        std::fs::write(&md_path, md_content)?;

        Ok((json_path, md_path))
    }

    /// Read all past changelog markdown summaries in chronological order.
    pub fn list_history(&self) -> std::io::Result<Vec<(String, String)>> {
        if !self.dir.exists() {
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();
        for entry in std::fs::read_dir(&self.dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                let content = std::fs::read_to_string(&path)?;
                entries.push((filename, content));
            }
        }

        entries.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(entries)
    }
}

fn sanitize_slug(input: &str) -> String {
    let alphanumeric_slug: String = input
        .chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect();
    let parts: Vec<&str> = alphanumeric_slug
        .split('_')
        .filter(|s| !s.is_empty())
        .take(5)
        .collect();
    parts.join("_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_change_dual_format() {
        let temp_dir = std::env::temp_dir().join(format!("test_cl_{}", uuid::Uuid::new_v4()));
        let cl = ChangelogManager::new(&temp_dir);

        let patch = ProfilePatch::default();
        let changes = vec!["Added story: DB Outage".to_string(), "Updated metrics".to_string()];
        let (json_path, md_path) = cl
            .record_change("Added DB incident story", &changes, &patch)
            .unwrap();

        assert!(json_path.exists());
        assert!(md_path.exists());

        let md_content = std::fs::read_to_string(&md_path).unwrap();
        assert!(md_content.contains("Added DB incident story"));
        assert!(md_content.contains("Added story: DB Outage"));

        let json_content = std::fs::read_to_string(&json_path).unwrap();
        let entry: ChangelogEntry = serde_json::from_str(&json_content).unwrap();
        assert_eq!(entry.message, "Added DB incident story");
        assert_eq!(entry.changes.len(), 2);

        let history = cl.list_history().unwrap();
        assert_eq!(history.len(), 1);

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
