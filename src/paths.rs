//! Default culprit paths and configuration overlay.

pub fn default_culprit_paths() -> Vec<&'static str> {
    vec![
        "~/Desktop",
        "~/Downloads",
        "~/Movies",
        "~/Library/Caches",
        "~/Library/Containers/*",
        "/Library/Developer",
        "~/Library/Developer",
        "~/Documents",
        "~/Pictures/Photos Library.photoslibrary",
        "~/Library/Application Support/Docker",
        "~/Library/Containers/com.docker.docker",
    ]
}
