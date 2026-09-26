use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use crate::schema::CuratedProfile;

/// Reads a CuratedProfile from either a path or standard input (`-`).
pub fn read_profile_state<P: AsRef<Path>>(path: P) -> io::Result<CuratedProfile> {
    let path_ref = path.as_ref();
    let content = if path_ref == Path::new("-") {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        std::fs::read_to_string(path_ref).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to read profile from '{}': {}", path_ref.display(), e),
            )
        })?
    };

    serde_json::from_str::<CuratedProfile>(&content).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse profile JSON: {}", e),
        )
    })
}

/// Atomically writes a CuratedProfile to disk (or standard output if `-`).
///
/// When writing to a file, writes to a sibling temporary file first and renames it,
/// ensuring concurrent readers never encounter half-written or corrupt states.
pub fn write_profile_state<P: AsRef<Path>>(path: P, profile: &CuratedProfile) -> io::Result<()> {
    let path_ref = path.as_ref();
    let serialized = serde_json::to_string_pretty(profile).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to serialize profile: {}", e),
        )
    })?;

    if path_ref == Path::new("-") {
        let mut stdout = io::stdout();
        stdout.write_all(serialized.as_bytes())?;
        stdout.write_all(b"\n")?;
        stdout.flush()?;
        return Ok(());
    }

    if let Some(parent) = path_ref.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }

    let temp_path = temp_file_path(path_ref);
    {
        let mut f = File::create(&temp_path)?;
        f.write_all(serialized.as_bytes())?;
        f.write_all(b"\n")?;
        f.flush()?;
        f.sync_all()?;
    }

    std::fs::rename(&temp_path, path_ref)?;
    Ok(())
}

fn temp_file_path(target: &Path) -> PathBuf {
    let file_name = target
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("profile.json");
    let random_id = uuid::Uuid::new_v4();
    let temp_name = format!(".{}.tmp-{}", file_name, random_id);
    match target.parent() {
        Some(parent) => parent.join(temp_name),
        None => PathBuf::from(temp_name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_write_and_read() {
        let temp_dir = std::env::temp_dir().join(format!("test_io_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir).unwrap();
        let file_path = temp_dir.join("profile.json");

        let mut profile = CuratedProfile::default();
        profile.contact.name = Some("Alice Smith".to_string());

        write_profile_state(&file_path, &profile).unwrap();
        assert!(file_path.exists());

        let read_back = read_profile_state(&file_path).unwrap();
        assert_eq!(read_back.contact.name.as_deref(), Some("Alice Smith"));

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
