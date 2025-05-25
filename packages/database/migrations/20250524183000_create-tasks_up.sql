CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY,
    action TEXT NOT NULL,
    status TEXT NOT NULL,
    retry INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS tasks_unique_waiting_action ON tasks (action)
WHERE
    status = '{"type": "waiting"}';
