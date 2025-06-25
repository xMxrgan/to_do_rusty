CREATE TABLE IF NOT EXISTS tasks (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    description TEXT,
    priority    INTEGER CHECK(priority >= 1 AND priority <= 5),
    status      TEXT CHECK(status IN ('active', 'completed', 'deleted')) DEFAULT 'active'
);
