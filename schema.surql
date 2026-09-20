-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- FUNCTIONS
-- ------------------------------

DEFINE FUNCTION fn::get_bucket_extras($ids: array<string>) -> array<{ extra: object, id: string }> { RETURN (SELECT VALUE { id: record::id(id), extra: extra } FROM array::filter(array::map($ids, |$k: any| type::record('summary_bucket', $k)), |$r: any| record::exists($r))) } PERMISSIONS FULL;
DEFINE FUNCTION fn::get_buckets_for_day($themes: array<string>, $end_timestamp: datetime) -> array<record<summary_bucket>> { RETURN array::filter(array::map($themes, |$theme: any| { (SELECT VALUE id FROM summary_bucket WHERE theme = $theme AND end_timestamp > $end_timestamp - 1d AND begin_timestamp < $end_timestamp ORDER BY end_timestamp DESC LIMIT 1)[0] }), |$b: any| $b != NONE) } COMMENT "returns the ids of buckets that have been created within a day of the timestamp which summarize the given themes. each theme's index in the input corresponds with the index of its bucket in the output." PERMISSIONS FULL;
DEFINE FUNCTION fn::get_daily_digest($themes: array<string>, $end_timestamp: datetime, $limit: none | int, $threshold: float) -> array<{ begin_timestamp: datetime, end_timestamp: datetime, id: string, sources: array<string>, summary: string, theme: string }> { RETURN array::map(fn::get_buckets_for_day($themes, $end_timestamp), |$bucket: any| (SELECT VALUE { id: record::id(id), theme: theme, summary: summary, begin_timestamp: begin_timestamp, end_timestamp: end_timestamp, sources: array::map(fn::get_events_in_bucket($bucket, $limit, $threshold), |$e: any| record::id($e)) } FROM ONLY $bucket)) } PERMISSIONS FULL;
DEFINE FUNCTION fn::get_event_extras($ids: array<string>) -> array<{ extra: object, id: string }> { RETURN (SELECT VALUE { id: record::id(id), extra: extra } FROM array::filter(array::map($ids, |$k: any| type::record('calendar_event', $k)), |$r: any| record::exists($r))) } PERMISSIONS FULL;
DEFINE FUNCTION fn::get_events_from_partial_ids($ids: array<string>) -> array<{ article_url: string, id: string, name: string, similarities: object, summary: string, themes: array<string>, timestamp: datetime }> { RETURN (SELECT VALUE { id: record::id(id), name: name, article_url: article_url, summary: summary, themes: <array> themes, similarities: similarities, timestamp: timestamp } FROM array::filter(array::map($ids, |$k: any| type::record('calendar_event', $k)), |$r: any| record::exists($r))) } PERMISSIONS FULL;
DEFINE FUNCTION fn::get_events_in_bucket($bucket: record<summary_bucket>, $limit: none | int, $threshold: float) -> array<record<calendar_event>> { LET $b = (SELECT * FROM ONLY $bucket); LET $rows = (SELECT id, similarities[$b.theme] ?? 0 AS score, timestamp FROM calendar_event WITH NOINDEX WHERE themes CONTAINS $b.theme AND timestamp >= $b.begin_timestamp AND timestamp <= $b.end_timestamp AND (similarities[$b.theme] ?? 0) >= $threshold ORDER BY score DESC, timestamp DESC); LET $ids = $rows.id; RETURN (IF $limit != NONE { array::slice($ids, 0, $limit) } ELSE { $ids }); } PERMISSIONS FULL;
DEFINE FUNCTION fn::get_events_with_any_theme($themes: array<string>, $threshold: float) -> array<{ article_url: string, id: string, name: string, similarities: object, summary: string, themes: array<string>, timestamp: datetime }> { RETURN (SELECT VALUE { id: record::id(id), name: name, article_url: article_url, summary: summary, themes: <array> themes, similarities: similarities, timestamp: timestamp } FROM calendar_event WITH NOINDEX WHERE themes CONTAINSANY $themes AND array::len(array::filter(object::entries(similarities), |$e: any| $themes CONTAINS $e[0] AND $e[1] >= $threshold)) > 0) } COMMENT 'Gets all events with at least one theme in common with this.\n\nTODO remove WITH NOINDEX when surrealdb bug is patched' PERMISSIONS FULL;
DEFINE FUNCTION fn::get_events_with_theme($theme: string) -> array<{ article_url: string, id: string, name: string, similarities: object, summary: string, themes: array<string>, timestamp: datetime }> { RETURN (SELECT VALUE { id: record::id(id), name: name, article_url: article_url, summary: summary, themes: <array> themes, similarities: similarities, timestamp: timestamp } FROM calendar_event WITH NOINDEX WHERE themes CONTAINS $theme) } COMMENT 'TODO remove WITH NOINDEX when the surrealdb bug is patched' PERMISSIONS FULL;

-- ------------------------------
-- TABLE: calendar_event
-- ------------------------------

DEFINE TABLE calendar_event TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD OVERWRITE article_url ON calendar_event TYPE string PERMISSIONS FULL;
DEFINE FIELD OVERWRITE extra ON calendar_event TYPE object FLEXIBLE DEFAULT {  } PERMISSIONS FULL;
DEFINE FIELD OVERWRITE name ON calendar_event TYPE string PERMISSIONS FULL;
DEFINE FIELD OVERWRITE similarities ON calendar_event TYPE object FLEXIBLE PERMISSIONS FULL;
DEFINE FIELD OVERWRITE summary ON calendar_event TYPE string PERMISSIONS FULL;
DEFINE FIELD OVERWRITE themes ON calendar_event TYPE set<string> VALUE <set> object::keys(similarities) PERMISSIONS FULL;
DEFINE FIELD OVERWRITE timestamp ON calendar_event TYPE datetime PERMISSIONS FULL;

DEFINE INDEX calendar_event_themes ON calendar_event FIELDS themes.*;
DEFINE INDEX calendar_event_themes_timestamp ON calendar_event FIELDS themes.*, timestamp;

DEFINE EVENT sync_themes ON calendar_event WHEN $event INSIDE ['CREATE', 'UPDATE', 'DELETE'] AND $before.themes != $after.themes THEN { FOR $t IN <array> set::union($before.themes ?? <set> [], $after.themes ?? <set> []) { LET $n = (SELECT count() FROM calendar_event WITH NOINDEX WHERE themes CONTAINS $t GROUP ALL)[0].count ?? 0; IF $n > 0 { UPSERT type::record('theme', $t) SET name = $t, event_count = $n } ELSE { DELETE type::record('theme', $t) }; } };

-- ------------------------------
-- TABLE: summary_bucket
-- ------------------------------

DEFINE TABLE summary_bucket TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD OVERWRITE begin_timestamp ON summary_bucket TYPE datetime PERMISSIONS FULL;
DEFINE FIELD OVERWRITE end_timestamp ON summary_bucket TYPE datetime PERMISSIONS FULL;
DEFINE FIELD OVERWRITE extra ON summary_bucket TYPE object FLEXIBLE DEFAULT {  } PERMISSIONS FULL;
DEFINE FIELD OVERWRITE summary ON summary_bucket TYPE string PERMISSIONS FULL;
DEFINE FIELD OVERWRITE theme ON summary_bucket TYPE string PERMISSIONS FULL;

DEFINE INDEX summary_bucket_theme ON summary_bucket FIELDS theme;
DEFINE INDEX summary_bucket_theme_end ON summary_bucket FIELDS theme, end_timestamp;


-- ------------------------------
-- TABLE: theme
-- ------------------------------

DEFINE TABLE theme TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD OVERWRITE event_count ON theme TYPE int PERMISSIONS FULL;
DEFINE FIELD OVERWRITE name ON theme TYPE string READONLY ASSERT $value == string::lowercase($value) PERMISSIONS FULL;



