CREATE TABLE IF NOT EXISTS tvshow_episode_files (
    file_id INTEGER NOT NULL REFERENCES files (id) ON DELETE CASCADE,
    episode_id INTEGER NOT NULL REFERENCES tvshow_episodes (id) ON DELETE CASCADE,
    PRIMARY KEY (file_id, episode_id)
);
