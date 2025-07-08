//! Compares snapshots to identify changes.

use crate::snapshot::Snapshot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Diff {
    pub new_files: Vec<String>,
    pub deleted_files: Vec<String>,
    pub changed_files: Vec<(String, u64, u64)>, // (path, old_size, new_size)
}

pub fn compare_snapshots(old: &Snapshot, new: &Snapshot) -> Diff {
    let mut new_files = Vec::new();
    let mut deleted_files = Vec::new();
    let mut changed_files = Vec::new();

    for (path, new_info) in new {
        match old.get(path) {
            Some(old_info) => {
                if new_info.size != old_info.size {
                    changed_files.push((path.clone(), old_info.size, new_info.size));
                }
            }
            None => {
                new_files.push(path.clone());
            }
        }
    }

    for (path, _) in old {
        if !new.contains_key(path) {
            deleted_files.push(path.clone());
        }
    }

    Diff {
        new_files,
        deleted_files,
        changed_files,
    }
}