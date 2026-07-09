// ────────────────────────────────────────────────────────────────────────────────
// Pag — Komenda wyszukiwania
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::Result;
use colored::*;

use crate::config::Config;
use crate::cli::Cli;
use crate::repo::sync;
use crate::utils::fmt;

pub async fn run(
    config: &Config,
    query: &[String],
    search_desc: bool,
    names_only: bool,
    cli: &Cli,
) -> Result<()> {
    let query_str = query.join(" ");

    // Wczytaj lokalne bazy
    let dbs = sync::load_all_dbs(&config.paths.db_path)?;

    if dbs.is_empty() {
        println!("{}", "No local databases. Run 'pag sync' first.".yellow());
        return Ok(());
    }

    println!("{} \"{}\"...", "Searching".bold(), query_str);

    let results = sync::search_packages(&query_str, &dbs, search_desc);

    if results.is_empty() {
        println!("  {}", "No packages found.".dimmed());
        return Ok(());
    }

    println!();

    if names_only {
        for (_repo, entry) in &results {
            println!("{}", entry.name.cyan());
        }
    } else {
        // Formatuj jako tabela
        let headers = ["Name", "Version", "Repository", "Description"];
        let rows: Vec<Vec<String>> = results
            .iter()
            .map(|(repo, entry)| {
                vec![
                    entry.name.clone(),
                    entry.version.clone(),
                    repo.name.clone(),
                    truncate(&entry.description, 60),
                ]
            })
            .collect();

        fmt::print_table(
            &headers,
            &rows,
            &[colored::Color::Cyan, colored::Color::Green, colored::Color::Yellow, colored::Color::White],
        );
    }

    println!();
    println!("{} results", results.len());

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
