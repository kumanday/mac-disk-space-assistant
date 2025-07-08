//! Manages the Rayon thread pool and work distribution.

use rayon::prelude::*;
use std::path::PathBuf;

pub fn process_paths<F, R>(paths: &[&str], task: F) -> Vec<R>
where
    F: Fn(PathBuf) -> R + Sync + Send,
    R: Send,
{
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get_physical())
        .build()
        .unwrap();

    pool.install(|| {
        paths
            .par_iter()
            .map(|path_str| {
                let path = PathBuf::from(path_str);
                task(path)
            })
            .collect()
    })
}
