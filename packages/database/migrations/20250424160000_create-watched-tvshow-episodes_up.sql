CREATE TABLE watched_tvshow_episodes (
    user_id INTEGER REFERENCES users (id) ON DELETE CASCADE,
    episode_id INTEGER NOT NULL REFERENCES tvshow_episodes (id) ON DELETE CASCADE,
    progress INTEGER DEFAULT 0,
    completed BOOL DEFAULT false,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, episode_id)
);
