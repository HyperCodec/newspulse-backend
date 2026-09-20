# Fuego

A high-performance news digest and event tracking API built with **Rust (Axum)**, **SurrealDB**, and **Python (`uv`)**. Fuego ingests structured semantic news data (`fuego-response.v1`), stores events and digest buckets in SurrealDB, and exposes REST and iCalendar (`.ics`) subscription endpoints.

---

## Features

* **Automated Data Ingestion:** Pure Python script managed via `uv` to parse, transform, and `UPSERT` news events and daily summary buckets into SurrealDB.
* **REST API:** Lightweight, asynchronous endpoints serving daily digests, themes, individual events, and detailed extra metadata (embeddings, semantic text, and direction data).
* **iCalendar Feeds:** Dynamic `.ics` stream endpoints (`/daily_digests.ics` and `/events.ics`) for direct integration into Apple Calendar, Google Calendar, or Outlook.
* **OpenAPI Specification:** Native OpenAPI spec available at `/openapi.json`.
* **Production-Ready Docker Container:** Optimized multi-stage build running on `debian:bookworm-slim` with TLS/WSS CA certificate support and dynamic `PORT` binding for cloud platforms like Vercel and Fly.io.

---

## Tech Stack

* **Backend:** Rust, Axum, Tokio, Utoipa, `icalendar`
* **Database:** SurrealDB (Namespace: `main`, Database: `main`)
* **Python / Scripting:** Python 3.10+, `uv`, `surrealdb` SDK, `python-dotenv`
* **Task Runner:** `just`

---

## Project Structure

```text
.
├── Cargo.toml          # Rust crate configuration
├── Cargo.lock
├── pyproject.toml      # Python project configuration (uv managed)
├── Justfile            # Task shortcuts for running, building, and ingesting
├── Dockerfile          # Multi-stage production container build
├── schema.surql        # SurrealDB database schema & custom functions
├── scripts/
│   └── ingest.py       # Python script for ingesting JSON data into SurrealDB
├── src/                # Axum backend source code
└── data/               # (Optional/Gitignored) Local JSON files for testing

```

---

## Prerequisites

* **Rust:** 1.75+ toolchain
* **uv:** Fast Python package installer (`curl -LsSf [https://astral.sh/uv/install.sh](https://astral.sh/uv/install.sh) | sh`)
* **Just:** Command runner (`cargo install just` or `brew install just`)
* **SurrealDB:** Running instance (local or hosted)

---

## Environment Configuration

Create a `.env` file in the project root:

```env
# SurrealDB Credentials
SURREAL_URI=ws://127.0.0.1:8000
SURREAL_USER=root
SURREAL_PASS=root

# Server Config (Optional - defaults to 0.0.0.0:3000 or $PORT if set)
PORT=3000
ADDR=0.0.0.0:3000

```

---

## Database Setup (`schema.surql`)

Before running the server or ingesting data, ensure your SurrealDB instance is initialized with `schema.surql`.

### Option A: Via SurrealDB CLI

```bash
surreal import \
  --conn http://127.0.0.1:8000 \
  --user root \
  --pass root \
  --ns main \
  --db main \
  schema.surql

```

### Option B: Via Surrealist / SurrealDB Studio

1. Open **Surrealist** (or your SurrealDB Studio web dashboard).
2. Connect to your namespace (`main`) and database (`main`).
3. Open a new Query tab, paste the contents of `schema.surql`, and click **Run**.

---

## Data Ingestion

The python ingestion tool parses `fuego-response.v1` JSON files, normalizes themes to lowercase (satisfying SurrealDB schema assertions), and upserts them atomically into `calendar_event` and `summary_bucket` tables.

### 1. Initialize Python Environment

```bash
just setup
# or directly: uv sync

```

### 2. Ingest Data

```bash
# Dry-run ingestion (validates and parses without writing to DB)
just ingest-dry data/sample.json

# Ingest specific JSON file(s)
just ingest data/fuego_output.json

# Ingest all JSON files in the data/ directory
just ingest-data

```

---

## Development Shortcuts (`Justfile`)

| Command | Description |
| --- | --- |
| `just setup` | Install/sync Python dependencies via `uv` |
| `just run` | Run the Rust server in development mode |
| `just build` | Build the release binary |
| `just check` | Run `cargo check` across the Rust project |
| `just ingest <files>` | Run the ingestion script on specified files |
| `just ingest-dry <files>` | Test schema mapping and output summary without writing |
| `just ingest-data` | Ingest all `.json` files inside `data/` |

---

## API Reference

### 1. General & System

* **`GET /openapi.json`**
Returns the auto-generated OpenAPI 3.0 specification.
* **`GET /themes`**
Returns all registered themes along with their article counts.

---

### 2. Events & Extra Data

* **`GET /event`**
Fetch one or more calendar events by their IDs.
* **Query Params:**
* `id`: Event ID (e.g., `?id=20260919211500-382&id=20260919210000-499`)




* **`GET /event/extra`**
Fetch full extra metadata for events (embeddings, semantic text, GDELT metadata).
* **Query Params:**
* `id`: Event ID (repeatable)




* **`GET /bucket/extra`**
Fetch full extra metadata for digest summary buckets (activity, direction, source bucket details).
* **Query Params:**
* `id`: Bucket ID (repeatable, e.g. `?id=bucket-sports_2026-09-19`)





---

### 3. Digests & Feeds

* **`GET /digest`**
Fetch the active daily news digest in JSON format.
* **Query Params:**
* `theme`: (Optional) Filter by theme name (repeatable, e.g. `?theme=sports&theme=health`)
* `limit`: (Optional) Max source articles per bucket
* `threshold`: (Optional) Minimum similarity score (default `0.0`)





---

### 4. iCalendar Subscription Streams (`.ics`)

Integrate directly into calendar client applications using these HTTP endpoints:

* **`GET /daily_digests.ics`**
iCalendar feed of daily digest summaries.
* **Query Params:**
* `title`: Calendar name (Required)
* `theme`: Filter by theme (repeatable)
* `limit`: Max sources per digest entry
* `threshold`: Minimum similarity score
* `days`: Number of historical days to include (default: `7`, max: `31`)




* **`GET /events.ics`**
iCalendar feed of individual articles/events.
* **Query Params:**
* `title`: Calendar name (Required)
* `theme`: Filter by theme (repeatable)
* `threshold`: Minimum similarity score





---

## Docker & Deployment

### Local Docker Build

```bash
# Build image
docker build -t fuego-server .

# Run container
docker run -d \
  -p 3000:3000 \
  -e SURREAL_URI="wss://your-surrealdb-host.com" \
  -e SURREAL_USER="root" \
  -e SURREAL_PASS="secret" \
  --name fuego-server \
  fuego-server

```

### Deploying to Cloud / Vercel / PaaS

The Rust server checks for the platform-injected `PORT` environment variable before falling back to `ADDR` or `0.0.0.0:3000`. This allows seamless deployment on PaaS platforms (Vercel, Render, Fly.io, Railway) that assign ports dynamically at runtime.