// ────────────────────────────────────────────────────────────────────────────────
// Pag — Formatowanie wyjścia
// ────────────────────────────────────────────────────────────────────────────────

use colored::*;
use std::fmt::Display;

/// Wydrukuj nagłówek operacji
pub fn print_header(title: &str) {
    println!("{}", format!(":: {}", title).blue().bold());
}

/// Wydrukuj sukces
pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

/// Wydrukuj błąd
pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg.red());
}

/// Wydrukuj ostrzeżenie
pub fn print_warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg.yellow());
}

/// Wydrukuj informację
pub fn print_info(msg: &str) {
    println!("  {}", msg.dimmed());
}

/// Formatuj tabelę z kolumnami
pub fn print_table(headers: &[&str], rows: &[Vec<String>], colors: &[Color]) {
    if rows.is_empty() {
        return;
    }

    let mut col_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();

    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < col_widths.len() {
                col_widths[i] = col_widths[i].max(cell.len());
            }
        }
    }

    // Nagłówek
    let header_line: Vec<String> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:<width$}", h.bold(), width = col_widths[i]))
        .collect();
    println!("{}", header_line.join("  "));

    // Separator
    let sep: Vec<String> = col_widths
        .iter()
        .map(|w| "─".repeat(*w))
        .collect();
    println!("{}", sep.join("  "));

    // Wiersze
    for row in rows {
        let line: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                let color = colors.get(i).unwrap_or(&Color::White);
                format!("{:<width$}", cell.color(*color), width = col_widths[i])
            })
            .collect();
        println!("{}", line.join("  "));
    }
}

/// Pasek postępu
pub struct ProgressBar {
    total: u64,
    current: u64,
    label: String,
}

impl ProgressBar {
    pub fn new(total: u64, label: &str) -> Self {
        ProgressBar {
            total,
            current: 0,
            label: label.to_string(),
        }
    }

    pub fn inc(&mut self, n: u64) {
        self.current += n;
        self.draw();
    }

    fn draw(&self) {
        if self.total == 0 {
            return;
        }
        let pct = (self.current as f64 / self.total as f64) * 100.0;
        let width = 40;
        let filled = ((self.current as f64 / self.total as f64) * width as f64) as usize;
        let empty = width - filled;

        print!(
            "\r  {} [{}{}] {:.1}%",
            self.label.dimmed(),
            "█".repeat(filled).green(),
            "░".repeat(empty),
            pct
        );
        use std::io::{self, Write};
        let _ = io::stdout().flush();
    }

    pub fn finish(&self) {
        println!();
    }
}

/// Lista pakietów z wersjami
pub fn print_package_list(packages: &[(String, String, String)], title: &str) {
    if packages.is_empty() {
        println!("  {}", "brak".dimmed());
        return;
    }

    println!("{}", title.blue().bold());
    let max_width = packages
        .iter()
        .map(|(name, _, _)| name.len())
        .max()
        .unwrap_or(0)
        .max(20);

    for (name, version, desc) in packages {
        println!(
            "  {:<name_width$} {:<version_width$} {}",
            name.cyan(),
            version.green(),
            desc.dimmed(),
            name_width = max_width + 2,
            version_width = 16
        );
    }
}
