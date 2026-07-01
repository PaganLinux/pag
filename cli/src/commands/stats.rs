// Komenda: stats
// Statystyki systemu

use crate::config::Config;
use colored::Colorize;

pub async fn execute(cfg: &Config) -> anyhow::Result<()> {
    let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;
    let stats = db.stats()?;

    println!("{} Statystyki systemu", "📊".bold());
    println!("  ┌─────────────────────────────────┐");
    println!("  │ Pakiety ogółem:     {:>10} │", stats.total_packages);
    println!("  │ ─ jawnie zainstal.: {:>10} │", stats.explicit_packages);
    println!("  │ ─ jako zależności:  {:>10} │", stats.dep_packages);
    println!("  │ Osierocone:         {:>10} │", stats.orphans);
    println!("  │ Plików:             {:>10} │", stats.total_files);
    println!("  │ Rozmiar:            {:>10} │", stats.total_size_human());
    println!("  └─────────────────────────────────┘");

    // Historia
    let history = db.get_history(5)?;
    if !history.is_empty() {
        println!("\n{} Ostatnie operacje:", "🕐".bold());
        for (ts, op, pkg, details) in &history {
            let dt = chrono::DateTime::from_timestamp(*ts, 0)
                .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_default();

            let op_icon = match op.as_str() {
                "install" => "📥",
                "remove" => "📤",
                "update" => "🔄",
                _ => "ℹ️",
            };

            println!("  {} {} {:30} {}", dt.dimmed(), op_icon, pkg.bold(), details);
        }
    }

    // Flatpaki
    if cfg.flatpak.enabled && crate::flatpak::is_flatpak_available() {
        if let Ok(flatpaks) = crate::flatpak::list_installed() {
            println!("\n{} Flatpaki: {}", "📦".bold(), flatpaks.len());
        }
    }

    Ok(())
}
