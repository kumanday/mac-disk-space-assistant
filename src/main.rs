#![forbid(unsafe_code)]

use clap::Parser;
use std::path::PathBuf;
use std::process::Command;

mod analyzer;
mod diff;
mod paths;
mod report;
mod scheduler;
mod scanner_docker;
mod scanner_fs;
mod snapshot;

/// Mac Disk Space Assistant
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Perform a deep scan of the filesystem
    Deep,
    /// Perform a daily scan of common culprit paths
    Daily,
    /// Install the daily scan as a launchd job
    InstallDaily,
}

fn main() {
    let cli = Cli::parse();

    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let mdsa_dir = home_dir.join(".mdsa");
    if !mdsa_dir.exists() {
        std::fs::create_dir_all(&mdsa_dir).expect("Could not create .mdsa directory");
    }
    let snapshot_path = mdsa_dir.join("state.json");


    match &cli.command {
        Commands::Deep => {
            println!("Running deep scan...");
            let paths = paths::default_culprit_paths();
            let snapshots = scheduler::process_paths(&paths, |path: PathBuf| {
                scanner_fs::scan_path(path)
            });

            let mut combined_snapshot = snapshot::Snapshot::new();
            for snapshot in snapshots {
                combined_snapshot.extend(snapshot);
            }

            snapshot::save_snapshot(&combined_snapshot, &snapshot_path);
            println!("Snapshot saved to {}", snapshot_path.display());

            let recommendations = analyzer::analyze_snapshot(&combined_snapshot);
            let paths_for_report: Vec<String> = combined_snapshot.keys().cloned().collect();
            report::generate_report(paths_for_report, recommendations);
            println!("Report generated at MDSA.md");
        }
        Commands::Daily => {
            println!("Running daily scan...");
            let old_snapshot = snapshot::load_snapshot(&snapshot_path).unwrap_or_default();

            let paths = paths::default_culprit_paths();
            let new_snapshots = scheduler::process_paths(&paths, |path: PathBuf| {
                scanner_fs::scan_path(path)
            });

            let mut new_snapshot = snapshot::Snapshot::new();
            for snapshot in new_snapshots {
                new_snapshot.extend(snapshot);
            }

            let diff = diff::compare_snapshots(&old_snapshot, &new_snapshot);

            snapshot::save_snapshot(&new_snapshot, &snapshot_path);
            println!("Snapshot saved to {}", snapshot_path.display());

            report::generate_diff_report(&diff);
            println!("Diff report generated at MDSA_diff.md");
        }
        Commands::InstallDaily => {
            println!("Installing daily scan...");
            let launch_agents_dir = home_dir.join("Library/LaunchAgents");
            if !launch_agents_dir.exists() {
                std::fs::create_dir_all(&launch_agents_dir).expect("Could not create LaunchAgents directory");
            }

            let plist_path = launch_agents_dir.join("com.mdsa.daily.plist");
            let plist_content = include_str!("../com.mdsa.daily.plist");

            // Get the current executable path and update the plist
            let current_exe = std::env::current_exe().unwrap();
            let updated_plist = plist_content.replace("/usr/local/bin/mdsa", current_exe.to_str().unwrap());

            std::fs::write(&plist_path, updated_plist).expect("Could not write plist file");

            Command::new("launchctl")
                .arg("load")
                .arg(&plist_path)
                .output()
                .expect("Failed to load launchd job");

            println!("Daily scan installed successfully.");
        }
    }
}
