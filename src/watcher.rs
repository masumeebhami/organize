use crate::organizer::organize_folder;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch_and_organize(path: PathBuf, dry_run: bool) {
    let (tx, rx) = channel::<Result<Event, notify::Error>>();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            let _ = tx.send(res);
        },
        Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .expect("Failed to create watcher");

    watcher
        .watch(&path, RecursiveMode::NonRecursive)
        .expect("Failed to watch directory");

    println!("Watching {:?}", path);

    for event in rx {
        match event {
            Ok(ev) => {
                println!("Filesystem event: {:?}", ev.kind);
                organize_folder(&path, dry_run);
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }
}
