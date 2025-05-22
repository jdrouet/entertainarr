CREATE TABLE IF NOT EXISTS tvshows (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    original_name TEXT NOT NULL,
    original_language TEXT NOT NULL,
    origin_country TEXT[] NOT NULL,
    overview TEXT,
    first_air_date INTEGER,
    poster_path TEXT,
    backdrop_path TEXT,
    popularity FLOAT NOT NULL,
    vote_count INTEGER NOT NULL,
    vote_average FLOAT NOT NULL,
    adult BOOLEAN NOT NULL,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);
