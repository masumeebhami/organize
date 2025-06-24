use chrono::{DateTime, Datelike, Utc};
use console::{Emoji, style};
use sha2::{Digest, Sha256};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub fn file_hash(path: &Path) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Some(format!("{:x}", hasher.finalize()))
}

pub fn find_nonconflicting_path(dir: &Path, file_name: &std::ffi::OsStr) -> PathBuf {
    let original = file_name.to_string_lossy().to_string();
    let mut path = dir.join(&original);

    if !path.exists() {
        return path;
    }

    let (base, ext) = match original.rsplit_once('.') {
        Some((b, e)) => (b.to_string(), format!(".{}", e)),
        None => (original.clone(), String::new()),
    };

    for i in 1..1000 {
        let candidate = format!("{} ({}){}", base, i, ext);
        path = dir.join(candidate);
        if !path.exists() {
            break;
        }
    }

    path
}
static FILE: Emoji<'_, '_> = Emoji("📄 ", "");
static CHECK: Emoji<'_, '_> = Emoji("✅", "✔");
static SKIP: Emoji<'_, '_> = Emoji("⏩", ">>");
static ERROR: Emoji<'_, '_> = Emoji("❌", "X");

pub fn print_move(from: &Path, to: &Path, dry_run: bool) {
    if dry_run {
        println!(
            "{} {} {} {}",
            style(SKIP).yellow(),
            FILE,
            style(from.display()).dim(),
            style("→").dim().bold(),
        );
        println!("   {}", style(to.display()).dim().italic());
    } else {
        println!(
            "{} {} {} {} {}",
            style(CHECK).green(),
            FILE,
            style(from.display()).green().bold(),
            style("→").green().bold(),
            style(to.display()).green()
        );
    }
}

pub fn print_error(msg: &str, path: &Path) {
    eprintln!(
        "{} {} {}",
        style(ERROR).red(),
        style("Failed").red().bold(),
        style(path.display()).red()
    );
    eprintln!("    {}", style(msg).red().italic());
}
pub fn extract_year_month(path: &Path) -> Option<(i32, u32)> {
    let metadata = fs::metadata(path).ok()?;
    let modified: SystemTime = metadata.modified().ok()?;
    let datetime: DateTime<Utc> = modified.into(); // Clear and explicit
    Some((datetime.year(), datetime.month()))
}
