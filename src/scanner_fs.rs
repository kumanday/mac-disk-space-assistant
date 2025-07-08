//! Filesystem metadata gathering, symlink resolution, and error bubbling.

use crate::snapshot::{FileInfo, Snapshot};
use glob::glob;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn scan_path(path: PathBuf) -> Snapshot {
    let mut snapshot = Snapshot::new();
    let expanded_path = shellexpand::tilde(&path.to_string_lossy()).to_string();

    if expanded_path.contains('*') {
        for entry in glob(&expanded_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    if path.is_dir() {
                        for entry in WalkDir::new(path) {
                            match entry {
                                Ok(entry) => {
                                    if entry.file_type().is_file() {
                                        let metadata = entry.metadata().unwrap();
                                        let file_info = FileInfo {
                                            size: metadata.len(),
                                            mtime: metadata.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                                        };
                                        snapshot.insert(entry.path().display().to_string(), file_info);
                                    }
                                }
                                Err(err) => {
                                    eprintln!("ERROR: {}", err);
                                }
                            }
                        }
                    } else {
                        let metadata = std::fs::metadata(&path).unwrap();
                        let file_info = FileInfo {
                            size: metadata.len(),
                            mtime: metadata.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                        };
                        snapshot.insert(path.display().to_string(), file_info);
                    }
                }
                Err(e) => eprintln!("ERROR: {:?}", e),
            }
        }
    } else {
        let path = PathBuf::from(expanded_path);
        if !path.exists() {
            eprintln!("ERROR: Path does not exist: {}", path.display());
            return snapshot;
        }

        for entry in WalkDir::new(path) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        let metadata = entry.metadata().unwrap();
                        let file_info = FileInfo {
                            size: metadata.len(),
                            mtime: metadata.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                        };
                        snapshot.insert(entry.path().display().to_string(), file_info);
                    }
                }
                Err(err) => {
                    eprintln!("ERROR: {}", err);
                }
            }
        }
    }
    snapshot
}