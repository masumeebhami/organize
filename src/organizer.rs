use crate::file_ops::{file_hash, find_nonconflicting_path};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn category_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        // Images
        ("jpg", "Images"),
        ("jpeg", "Images"),
        ("png", "Images"),
        ("gif", "Images"),
        ("bmp", "Images"),
        ("tiff", "Images"),
        ("webp", "Images"),
        ("heic", "Images"),
        ("svg", "Images"),
        ("dng", "Images"),
        // Videos
        ("mp4", "Videos"),
        ("mov", "Videos"),
        ("avi", "Videos"),
        ("mkv", "Videos"),
        ("webm", "Videos"),
        ("flv", "Videos"),
        ("wmv", "Videos"),
        // Audio
        ("mp3", "Audio"),
        ("wav", "Audio"),
        ("flac", "Audio"),
        ("aac", "Audio"),
        ("ogg", "Audio"),
        ("m4a", "Audio"),
        // Documents
        ("pdf", "Documents"),
        ("doc", "Documents"),
        ("docx", "Documents"),
        ("odt", "Documents"),
        ("rtf", "Documents"),
        ("txt", "Documents"),
        ("md", "Documents"),
        ("pages", "Documents"),
        // Presentations
        ("ppt", "Presentations"),
        ("pptx", "Presentations"),
        ("odp", "Presentations"),
        // Spreadsheets
        ("xls", "Spreadsheets"),
        ("xlsx", "Spreadsheets"),
        ("csv", "Spreadsheets"),
        ("ods", "Spreadsheets"),
        ("tsv", "Spreadsheets"),
        // Archives
        ("zip", "Archives"),
        ("rar", "Archives"),
        ("tar", "Archives"),
        ("gz", "Archives"),
        ("bz2", "Archives"),
        ("xz", "Archives"),
        ("7z", "Archives"),
        ("iso", "Archives"),
        // Code
        ("rs", "Code"),
        ("py", "Code"),
        ("js", "Code"),
        ("ts", "Code"),
        ("html", "Code"),
        ("css", "Code"),
        ("json", "Code"),
        ("xml", "Code"),
        ("java", "Code"),
        ("cpp", "Code"),
        ("c", "Code"),
        ("h", "Code"),
        ("cs", "Code"),
        ("go", "Code"),
        ("php", "Code"),
        ("sh", "Code"),
        ("bat", "Code"),
        ("toml", "Code"),
        ("yml", "Code"),
        ("yaml", "Code"),
        // Fonts
        ("ttf", "Fonts"),
        ("otf", "Fonts"),
        ("woff", "Fonts"),
        ("woff2", "Fonts"),
        // Executables
        ("exe", "Executables"),
        ("msi", "Executables"),
        ("apk", "Executables"),
        ("deb", "Executables"),
        ("rpm", "Executables"),
        ("app", "Applications"),
        // System & Config
        ("log", "System"),
        ("ini", "System"),
        ("cfg", "System"),
        // Disk Images
        ("dmg", "Disk Images"),
        ("img", "Disk Images"),
        // Misc
        ("db", "Databases"),
        ("sqlite", "Databases"),
        ("bak", "Backups"),
        // Calendar
        ("ics", "Calendars"),
        // Package
        ("pkg", "Packages"),
    ])
}

pub fn organize_folder(path: &PathBuf, dry_run: bool) {
    let categories = category_map();

    for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to read entry: {}", e);
                continue;
            }
        };

        let file_path = entry.path();

        if let Some(name) = file_path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with('.') {
                continue;
            }
        }

        if file_path.is_dir() && file_path.extension().and_then(|e| e.to_str()) == Some("app") {
            let target_dir = path.join("Applications");
            if let Err(e) = fs::create_dir_all(&target_dir) {
                eprintln!("Failed to create target dir: {}", e);
                continue;
            }

            if let Some(name) = file_path.file_name() {
                let target_path = find_nonconflicting_path(&target_dir, name);
                log_move(file_path, &target_path, dry_run);
            }
            continue;
        }

        if !file_path.is_file() {
            continue;
        }

        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            if let Some(subfolder) = categories.get(ext.to_lowercase().as_str()) {
                let target_dir = path.join(subfolder);
                if let Err(e) = fs::create_dir_all(&target_dir) {
                    eprintln!("Failed to create target dir: {}", e);
                    continue;
                }

                if let Some(name) = file_path.file_name() {
                    let mut target_path = target_dir.join(name);

                    if target_path.exists() {
                        let src_hash = file_hash(file_path);
                        let dst_hash = file_hash(&target_path);

                        if src_hash.is_some() && dst_hash.is_some() && src_hash == dst_hash {
                            println!("Duplicate file skipped: {:?}", file_path);
                            continue;
                        }

                        target_path = find_nonconflicting_path(&target_dir, name);
                    }

                    log_move(file_path, &target_path, dry_run);
                }
            }
        }
    }
}

fn log_move(from: &Path, to: &Path, dry_run: bool) {
    if dry_run {
        println!("[DRY RUN] Would move {:?} -> {:?}", from, to);
    } else if let Err(e) = fs::rename(from, to) {
        eprintln!("Failed to move {:?} → {:?}: {}", from, to, e);
    } else {
        println!("Moved {:?} → {:?}", from, to);
    }
}
