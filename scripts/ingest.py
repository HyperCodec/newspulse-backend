#!/usr/bin/env python3
"""Load fuego-response.v1 JSON files into SurrealDB (namespace "main", database "main").

    pip install "surrealdb>=2" python-dotenv
    python ingest.py fuego_output.json [more.json ...]
    python ingest.py --dry-run fuego_output.json

Reads SURREAL_URI, SURREAL_USER and SURREAL_PASS from the environment / .env.
Tested with surrealdb (Python SDK) 2.0.0 against SurrealDB 3.2.4.

Mapping
-------
article  -> calendar_event:<article id>
    title -> name, url -> article_url, published_at -> timestamp,
    buckets[] -> themes (set) + similarities ({theme: score}),
    source / semantic_text / embedding / metadata -> extra (one object column)
bucket   -> summary_bucket:<bucket id>_<UTC date of generated_at>
    one bucket per theme per day, so re-running on the same day updates it in place.
    window = the 24h ending at generated_at.
    everything else (description, activity, direction, ...) -> extra

The script is idempotent: ids are deterministic and writes are UPSERTs.
"""
import argparse
import json
import os
import sys
from datetime import datetime, timedelta, timezone

from dotenv import load_dotenv
from surrealdb import Surreal
from surrealdb.errors import SurrealError

NS = DB = "main"
DIGEST_WINDOW = timedelta(hours=24)  # bucket window = [generated_at - 24h, generated_at]
EXPECTED_SCHEMA = "fuego-response.v1"

# Timestamps travel as strings and are cast here; themes travel as a list and are cast to a set.
# Each of these is a single statement, so each batch is atomic.
EVENT_SQL = """
FOR $r IN $rows {
    UPSERT type::record('calendar_event', $r.id) CONTENT {
        name: $r.name,
        article_url: $r.article_url,
        summary: $r.summary,
        themes: <set>$r.themes,
        similarities: $r.similarities,
        timestamp: <datetime>$r.timestamp,
        extra: $r.extra
    };
};
"""

BUCKET_SQL = """
FOR $b IN $rows {
    UPSERT type::record('summary_bucket', $b.id) CONTENT {
        theme: $b.theme,
        summary: $b.summary,
        begin_timestamp: <datetime>$b.begin_timestamp,
        end_timestamp: <datetime>$b.end_timestamp,
        extra: $b.extra
    };
};
"""


# ── helpers ────────────────────────────────────────────────────────────────

def parse_ts(s: str) -> datetime:
    return datetime.fromisoformat(s.replace("Z", "+00:00")).astimezone(timezone.utc)


def iso(dt: datetime) -> str:
    return dt.isoformat().replace("+00:00", "Z")


def normalize_uri(uri: str) -> str:
    """The SDK needs a scheme and adds /rpc itself."""
    uri = uri.strip().rstrip("/")
    if uri.endswith("/rpc"):
        uri = uri[: -len("/rpc")]
    if uri.startswith("https://"):
        return "wss://" + uri[len("https://") :]
    if uri.startswith("http://"):
        return "ws://" + uri[len("http://") :]
    if "://" not in uri:
        # Match Rust's <Wss> client by defaulting to secure WebSocket
        if uri.startswith("localhost") or uri.startswith("127.0.0.1"):
            return "ws://" + uri
        return "wss://" + uri
    return uri


def chunks(items: list, size: int):
    for i in range(0, len(items), size):
        yield items[i : i + size]


# ── transform ──────────────────────────────────────────────────────────────

def transform(doc: dict, theme_key: str):
    if doc.get("schema_version") != EXPECTED_SCHEMA:
        print(
            f"warning: schema_version is {doc.get('schema_version')!r}, expected {EXPECTED_SCHEMA!r}",
            file=sys.stderr,
        )

    generated_at = parse_ts(doc["generated_at"])
    day = generated_at.date().isoformat()

    # Lowercase theme strings to satisfy SurrealDB schema/event constraints
    theme_of_bucket = {
        b["id"]: (b["name"] if theme_key == "name" else b["id"]).lower()
        for b in doc.get("buckets", [])
    }

    def theme_for(bucket_id: str) -> str:
        if bucket_id not in theme_of_bucket:
            print(
                f"warning: article references unknown bucket {bucket_id!r}; using the id as its theme",
                file=sys.stderr,
            )
            return bucket_id.lower()
        return theme_of_bucket[bucket_id]

    events = []
    for a in doc.get("articles", []):
        sims = {theme_for(m["bucket_id"]): float(m["similarity"]) for m in a.get("buckets", [])}
        events.append({
            "id": a["id"],
            "name": a["title"],
            "article_url": a["url"],
            "summary": a.get("summary") or "",
            "themes": sorted(sims),
            "similarities": sims,
            "timestamp": iso(parse_ts(a["published_at"])),
            "extra": {
                "source": a.get("source"),
                "semantic_text": a.get("semantic_text"),
                "embedding": a.get("embedding"),
                "metadata": a.get("metadata"),
            },
        })

    buckets = []
    for b in doc.get("buckets", []):
        buckets.append({
            "id": f"{b['id']}_{day}",
            "theme": theme_of_bucket[b["id"]],
            "summary": b.get("summary") or "",
            "begin_timestamp": iso(generated_at - DIGEST_WINDOW),
            "end_timestamp": iso(generated_at),
            "extra": {
                "source_bucket_id": b["id"],
                "type": b.get("type"),
                "name": b.get("name"),
                "description": b.get("description"),
                "activity": b.get("activity"),
                "direction": b.get("direction"),
                "generated_at": doc["generated_at"],
            },
        })

    return events, buckets


# ── database ───────────────────────────────────────────────────────────────

def sign_in(db, user: str, password: str) -> None:
    try:  # a database-level user (like the app's `server` user)
        db.signin({"namespace": NS, "database": DB, "username": user, "password": password})
    except SurrealError:  # otherwise a root user
        db.signin({"username": user, "password": password})
    db.use(NS, DB)


def write(db, sql: str, rows: list) -> None:
    # The SDK raises (e.g. InternalError) if any statement fails, such as a schema violation.
    db.query(sql, {"rows": rows})


# ── main ───────────────────────────────────────────────────────────────────

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("files", nargs="+", help="fuego-response.v1 JSON files")
    ap.add_argument("--batch-size", type=int, default=50, help="records per query (default 50)")
    ap.add_argument("--theme-key", choices=("name", "id"), default="name",
                    help="use bucket name ('Sports') or id ('bucket-sports') as the theme string (default: name)")
    ap.add_argument("--dry-run", action="store_true", help="transform and report, but don't write")
    args = ap.parse_args()

    loaded = []
    for path in args.files:
        with open(path, encoding="utf-8") as f:
            events, buckets = transform(json.load(f), args.theme_key)
        themes = sorted({t for e in events for t in e["themes"]})
        print(f"{path}: {len(events)} events, {len(buckets)} buckets, themes={themes}")
        loaded.append((events, buckets))

    if args.dry_run:
        return 0

    load_dotenv()
    try:
        uri, user, password = (os.environ[k] for k in ("SURREAL_URI", "SURREAL_USER", "SURREAL_PASS"))
    except KeyError as e:
        print(f"missing environment variable {e}", file=sys.stderr)
        return 1

    with Surreal(normalize_uri(uri)) as db:
        sign_in(db, user, password)
        for (events, buckets), path in zip(loaded, args.files):
            for batch in chunks(events, args.batch_size):
                write(db, EVENT_SQL, batch)
            for batch in chunks(buckets, args.batch_size):
                write(db, BUCKET_SQL, batch)
            print(f"{path}: written")

    return 0


if __name__ == "__main__":
    sys.exit(main())
