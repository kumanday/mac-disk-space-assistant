//! Handles the creation and persistence of filesystem snapshots.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub size: u64,
    pub mtime: i64,
}

pub type Snapshot = HashMap<String, FileInfo>;

pub fn save_snapshot(snapshot: &Snapshot, path: &Path) {
    let json = serde_json::to_string_pretty(snapshot).expect("Failed to serialize snapshot");
    fs::write(path, json).expect("Failed to write snapshot");
}

pub fn load_snapshot(path: &Path) -> Option<Snapshot> {
    if !path.exists() {
        return None;
    }
    let json = fs::read_to_string(path).expect("Failed to read snapshot");
    let snapshot = serde_json::from_str(&json).expect("Failed to deserialize snapshot");
    Some(snapshot)
}