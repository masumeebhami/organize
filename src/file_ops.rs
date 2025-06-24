use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

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
