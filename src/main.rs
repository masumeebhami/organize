use crate::organizer::organize_folder;
use crate::watcher::watch_and_organize;
use clap::Parser;
use std::path::PathBuf;

mod file_ops;
mod organizer;
mod watcher;

#[derive(Parser)]
#[command(name = "organize")]
#[command(about = "Smart file organizer for macOS", long_about = None)]
struct Args {
    /// Directory to watch
    #[arg(short, long, default_value = "~/Downloads")]
    dir: String,

    /// Run once instead of watching
    #[arg(short, long)]
    once: bool,

    /// Preview changes without moving files
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();
    let expanded = shellexpand::tilde(&args.dir).to_string();
    let path = PathBuf::from(expanded);

    if args.once {
        organize_folder(&path, args.dry_run);
    } else {
        watch_and_organize(path, args.dry_run);
    }
}
