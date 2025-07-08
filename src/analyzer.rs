//! Analyzes scanned data and applies heuristics.

use crate::snapshot::Snapshot;

pub struct Recommendation {
    pub path: String,
    pub size: u64,
    pub reason: String,
}

pub fn analyze_snapshot(snapshot: &Snapshot) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();
    for (path, file_info) in snapshot {
        if file_info.size > 1_000_000_000 {
            recommendations.push(Recommendation {
                path: path.clone(),
                size: file_info.size,
                reason: "File is larger than 1GB".to_string(),
            });
        }
    }
    recommendations
}