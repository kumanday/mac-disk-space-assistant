//! Generates Markdown reports.

use crate::analyzer::Recommendation;
use crate::diff::Diff;
use pulldown_cmark::{html, Options, Parser};
use std::fs;

pub fn generate_report(paths: Vec<String>, recommendations: Vec<Recommendation>) {
    let mut markdown_input = String::new();
    markdown_input.push_str("# Mac Disk Space Assistant Report\n\n");

    if !recommendations.is_empty() {
        markdown_input.push_str("## Top Recommendations\n\n");
        markdown_input.push_str("| Rank | Path | Size | Why |\n");
        markdown_input.push_str("|---|---|---|---|\n");
        for (i, rec) in recommendations.iter().enumerate() {
            markdown_input.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                i + 1,
                rec.path,
                rec.size,
                rec.reason
            ));
        }
        markdown_input.push_str("\n");
    }

    markdown_input.push_str("## Scanned Paths\n\n");

    for path in paths {
        markdown_input.push_str(&format!("- {}\n", path));
    }

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    fs::write("MDSA.md", &markdown_input).expect("Unable to write report");
}

pub fn generate_diff_report(diff: &Diff) {
    let mut markdown_input = String::new();
    markdown_input.push_str("# Mac Disk Space Assistant - Daily Diff Report\n\n");

    if !diff.new_files.is_empty() {
        markdown_input.push_str("## New Files\n\n");
        for path in &diff.new_files {
            markdown_input.push_str(&format!("- {}\n", path));
        }
        markdown_input.push_str("\n");
    }

    if !diff.deleted_files.is_empty() {
        markdown_input.push_str("## Deleted Files\n\n");
        for path in &diff.deleted_files {
            markdown_input.push_str(&format!("- {}\n", path));
        }
        markdown_input.push_str("\n");
    }

    if !diff.changed_files.is_empty() {
        markdown_input.push_str("## Changed Files\n\n");
        markdown_input.push_str("| Path | Old Size | New Size |\n");
        markdown_input.push_str("|---|---|---|\n");
        for (path, old_size, new_size) in &diff.changed_files {
            markdown_input.push_str(&format!("| {} | {} | {} |\n", path, old_size, new_size));
        }
        markdown_input.push_str("\n");
    }

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    fs::write("MDSA_diff.md", &markdown_input).expect("Unable to write diff report");
}