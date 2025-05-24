CREATE TABLE IF NOT EXISTS files (
    id INTEGER PRIMARY KEY,
    store TEXT NOT NULL,
    path TEXT NOT NULL,
    size INTEGER NOT NULL,
    content_type TEXT,
    created_at INTEGER NOT NULL,
    modified_at INTEGER NOT NULL,
    UNIQUE (store, path)
);
