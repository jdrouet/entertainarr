CREATE TABLE followed_tvshows (
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    tvshow_id INTEGER NOT NULL REFERENCES tvshows (id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, tvshow_id)
);
