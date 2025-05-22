CREATE TABLE IF NOT EXISTS tvshow_episodes (
    id INTEGER PRIMARY KEY,
    season_id INTEGER NOT NULL REFERENCES tvshow_seasons (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    air_date INTEGER,
    overview TEXT,
    episode_number INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);
