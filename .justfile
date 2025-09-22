# Justfile for Smart File Organizer

# Usage: just once <directory>
once dir:
    @echo "Running organize once on {{dir}}"
    cargo run -- --once --dir "{{dir}}"

# Usage: just watch <directory>
watch dir:
    @echo "Watching directory {{dir}}"
    cargo run -- --dir "{{dir}}"

# Usage: just dry-run <directory>
dry-run dir:
    @echo "Dry run organizing {{dir}}"
    cargo run -- --once --dry-run --dir "{{dir}}"

# Default: list all recipes
default:
    @just --list

format:
    cargo fmt

clippy:
    cargo clippy

check:
    cargo check