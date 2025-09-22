use chrono::{DateTime, Datelike, Utc};
use console::{Emoji, style};
use sha2::{Digest, Sha256};
use std::ffi::{OsStr, OsString};
use std::fs::{self, File, Metadata};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

static FILE_: Emoji<'_, '_> = Emoji("📄 ", "");
static CHECK: Emoji<'_, '_> = Emoji("✅", "✔");
static SKIP: Emoji<'_, '_> = Emoji("⏩", ">>");
static ERROR: Emoji<'_, '_> = Emoji("❌", "X");

/// Compute a SHA-256 hash of a file at `path`.
/// Returns an io::Result so callers can surface real errors instead of a silent None.
pub fn file_hash(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();

    // Larger buffer for fewer syscalls; BufReader isn’t necessary when we already
    // manage the read loop with our own buffer.
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Given a directory and a desired filename (OsStr), find a non-conflicting path by
/// appending " (n)" before the extension, e.g. "name (1).ext".
/// Avoids lossy UTF-8 conversions and respects platform path semantics.
pub fn find_nonconflicting_path(dir: &Path, file_name: &OsStr) -> PathBuf {
    let mut candidate = dir.join(file_name);
    if !candidate.exists() {
        return candidate;
    }

    // Split stem & extension safely (no UTF-8 assumptions)
    let stem = Path::new(file_name)
        .file_stem()
        .map(OsStr::to_os_string)
        .unwrap_or_else(|| OsString::from(file_name));
    let ext = Path::new(file_name).extension().map(OsStr::to_os_string);

    // Build "{stem} (n){.ext?}"
    let mut i: u32 = 1;
    loop {
        let mut next = OsString::new();
        next.push(&stem);
        next.push(format!(" ({})", i));
        if let Some(ref e) = ext {
            next.push(".");
            next.push(e);
        }

        candidate = dir.join(&next);
        if !candidate.exists() {
            return candidate;
        }
        i = i.saturating_add(1); // avoid overflow panic (practically unreachable)
    }
}

pub fn print_move(from: &Path, to: &Path, dry_run: bool) {
    let arrow = "→";
    if dry_run {
        println!(
            "{} {} {} {}",
            style(SKIP).yellow(),
            FILE_,
            style(from.display()).dim(),
            style(arrow).dim().bold(),
        );
        println!("   {}", style(to.display()).dim().italic());
    } else {
        println!(
            "{} {} {} {} {}",
            style(CHECK).green(),
            FILE_,
            style(from.display()).green().bold(),
            style(arrow).green().bold(),
            style(to.display()).green()
        );
    }
}

/// Print a formatted error with context and (optionally) a source error.
pub fn print_error(msg: &str, path: &Path, source: Option<&dyn std::error::Error>) {
    eprintln!(
        "{} {} {}",
        style(ERROR).red(),
        style("Failed").red().bold(),
        style(path.display()).red()
    );
    eprintln!("    {}", style(msg).red().italic());
    if let Some(err) = source {
        eprintln!("    {}", style(format!("caused by: {}", err)).red().dim());
    }
}

/// Extract (year, month) from a file's timestamp, preferring `created()`
/// and falling back to `modified()` when `created()` is unsupported.
pub fn extract_year_month(path: &Path) -> io::Result<(i32, u32)> {
    let meta = fs::metadata(path)?;
    let t = preferred_file_time(&meta)?;
    let dt: DateTime<Utc> = t.into();
    Ok((dt.year(), dt.month()))
}

/// Pick the most stable timestamp available for dating files.
/// On many Unix FSes, `created()` may be unsupported—handle that gracefully.
fn preferred_file_time(meta: &Metadata) -> io::Result<SystemTime> {
    match meta.created() {
        Ok(c) => Ok(c),
        Err(_) => meta.modified(),
    }
}
