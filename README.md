# 📂 Smart File Organizer (Rust + macOS)

A powerful, customizable CLI + watcher tool written in **Rust** to automatically **sort and organize files** on your Mac.

✅ Categorize files by type (images, documents, videos, apps, etc.)  
✅ Detect and handle duplicates using file hashing  
✅ Auto-rename conflicting files  
✅ Watch folders for live changes  
✅ Supports `.app` bundles (macOS applications)  
✅ Built with testable, modular architecture

---

## ✨ Features

- 🔍 **Dry-run mode** — preview actions before moving files.
- 📁 **Category-based subfoldering** — organize by file type automatically.
- ♻️ **Duplicate detection** — skip or overwrite identical files.
- 👀 **Live folder watching** — powered by [`notify`](https://crates.io/crates/notify).
- 🧠 **Fully testable logic** — using `lib.rs` and modular code.
- 🖼️ Works with `.app` bundles and other macOS-specific formats.

---

## 🛠️ Installation

### 1. Clone the repo

```bash
git clone https://github.com/yourusername/organize-rs.git
cd organize-rs
```
2. Build the binary
```bash
cargo build --release
```
Binary will be located in: target/release/organize

🚀 Usage
📁 Run Once
Organize your Downloads folder just once:

```bash
cargo run -- --once --dir ~/Downloads
```
🧪 Dry Run Mode
Preview what the tool would do, without changing anything:

```bash
cargo run -- --once --dry-run --dir ~/Desktop
```
👀 Watch Mode (Live Monitoring)
Continuously watch a folder and organize new files:

```bash
cargo run -- --dir ~/Downloads
```
📂 Example Output
```
Moved "/Users/me/Downloads/photo.jpg" → "/Users/me/Downloads/Images/photo.jpg"
[DRY RUN] Would move "/Users/me/Desktop/file.txt" → "/Users/me/Desktop/Documents/file.txt"
Skipping duplicate: "/Users/me/Downloads/receipt.pdf"
```
🧪 Running Tests
```bash
cargo test
```
Make sure src/lib.rs exists and exposes your modules:

```bash
pub mod file_ops;
pub mod organizer;
```
Tests live in tests/organizer_tests.rs.


🔧 File Type Categories
Supports:
```
🖼️ Images: jpg, png, heic, webp, tiff, etc.

📄 Documents: pdf, docx, txt, md, etc.

🎞️ Videos: mp4, mov, webm, mkv, etc.

🎧 Audio: mp3, wav, flac, ogg, etc.

📦 Archives: zip, tar, gz, 7z, etc.

🧑‍💻 Code: rs, py, js, ts, html, css, etc.

💾 Apps: .app bundles (macOS only)

...and more!
```

🤝 Contributing
Feel free to fork, improve, or submit PRs!
Bug reports and feedback welcome via GitHub Issues.
We will update it to ensure full compatibility with Intel-based Macs.