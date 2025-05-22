CREATE TABLE IF NOT EXISTS tvshow_seasons (
    id INTEGER PRIMARY KEY,
    tvshow_id INTEGER NOT NULL REFERENCES tvshows (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    air_date INTEGER,
    overview TEXT,
    poster_path TEXT,
    season_number INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);
