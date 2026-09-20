set dotenv-load := true

# List available recipes
default:
    @just --list

# Sync Python dependencies with uv
setup:
    uv sync
    cargo fetch

# Run data ingestion on one or more JSON files (e.g., `just ingest data/sample.json`)
ingest +FILES:
    uv run python scripts/ingest.py {{FILES}}

# Dry-run data ingestion without writing to SurrealDB
ingest-dry +FILES:
    uv run python scripts/ingest.py --dry-run {{FILES}}

# Ingest all JSON files in data/ directory
ingest-data:
    uv run python scripts/ingest.py data/*.json

# --- Rust Workflow ---

# Run the Rust project
run:
    cargo run

# Build release binary
build:
    cargo build --release

# Check Rust project for errors
check:
    cargo check