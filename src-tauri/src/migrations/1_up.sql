CREATE TABLE IF NOT EXISTS games (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        executable TEXT NOT NULL,
        description TEXT,
        image TEXT
);