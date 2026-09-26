use std::fs;
use std::process::Command;

#[test]
fn test_cli_init_and_guide() {
    let temp_dir = std::env::temp_dir().join(format!("test_cli_ig_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let state_file = temp_dir.join("profile.json");

    // 1. Run profile-curator init --state profile.json
    let bin_path = env!("CARGO_BIN_EXE_profile-curator");
    let status_init = Command::new(bin_path)
        .args(["init", "--output", state_file.to_str().unwrap()])
        .status()
        .expect("Failed to execute init command");
    assert!(status_init.success());
    assert!(state_file.exists());

    // 2. Run profile-curator guide --state profile.json
    let output_guide = Command::new(bin_path)
        .args(["guide", "--state", state_file.to_str().unwrap(), "--json"])
        .output()
        .expect("Failed to execute guide command");
    assert!(output_guide.status.success());
    let guide_json: serde_json::Value =
        serde_json::from_slice(&output_guide.stdout).expect("Valid JSON guide output");
    assert!(guide_json.get("suggested_questions").is_some());
    assert!(guide_json.get("readiness_score").is_some());

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_cli_patch_and_changelog() {
    let temp_dir = std::env::temp_dir().join(format!("test_cli_pc_{}", uuid::Uuid::new_v4()));
    let changelog_dir = temp_dir.join("history");
    fs::create_dir_all(&temp_dir).unwrap();
    let state_file = temp_dir.join("profile.json");

    let bin_path = env!("CARGO_BIN_EXE_profile-curator");

    // Initialize with sample
    let status_init = Command::new(bin_path)
        .args([
            "init",
            "--sample",
            "--output",
            state_file.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute init");
    assert!(status_init.success());

    // Patch: Add a new interview story and log changelog
    let patch_json = r#"{
        "add_stories": [
            {
                "id": "story-cli-test",
                "title": "Scaling Distributed Cache",
                "situation": "Cache node contention in cluster",
                "task": "Redesign hashing strategy",
                "action": "Implemented consistent hashing with virtual nodes",
                "result": "Zero hotspots and 40% memory rebalance efficiency",
                "tags": ["cache", "scaling"]
            }
        ]
    }"#;

    let status_patch = Command::new(bin_path)
        .args([
            "patch",
            "--state",
            state_file.to_str().unwrap(),
            "--patch-json",
            patch_json,
            "--changelog-dir",
            changelog_dir.to_str().unwrap(),
            "--message",
            "Added distributed cache story from interview turn 1",
        ])
        .status()
        .expect("Failed to execute patch");
    assert!(status_patch.success());

    // Verify changelog generated
    assert!(changelog_dir.exists());
    let changelog_files: Vec<_> = fs::read_dir(&changelog_dir)
        .unwrap()
        .map(|r| r.unwrap().path())
        .collect();
    assert!(changelog_files.iter().any(|p| p.extension().and_then(|s| s.to_str()) == Some("json")));
    assert!(changelog_files.iter().any(|p| p.extension().and_then(|s| s.to_str()) == Some("md")));

    // Verify history subcommand
    let output_history = Command::new(bin_path)
        .args(["history", "--changelog-dir", changelog_dir.to_str().unwrap()])
        .output()
        .expect("Failed to run history");
    assert!(output_history.status.success());
    let history_text = String::from_utf8_lossy(&output_history.stdout);
    assert!(history_text.contains("Added distributed cache story"));

    // Verify export subcommand
    let cv_file = temp_dir.join("cv.json");
    let status_export = Command::new(bin_path)
        .args([
            "export",
            "--state",
            state_file.to_str().unwrap(),
            "--output",
            cv_file.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to run export");
    assert!(status_export.success());
    assert!(cv_file.exists());

    let _ = fs::remove_dir_all(&temp_dir);
}
