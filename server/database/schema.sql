-- Table to store game states
CREATE TABLE IF NOT EXISTS Game (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    phase TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    initialized_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_game_timestamp ON Game (timestamp);

-- Table to store ledger entries for GameState transactions
CREATE TABLE IF NOT EXISTS Ledger (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    command TEXT NOT NULL,
    payload TEXT NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Game (id),
    FOREIGN KEY (player_id) REFERENCES Player (id)
);

CREATE INDEX IF NOT EXISTS idx_ledger_game_id ON Ledger (game_id);
CREATE INDEX IF NOT EXISTS idx_ledger_timestamp ON Ledger (timestamp);

CREATE TABLE IF NOT EXISTS Player (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    creatures TEXT DEFAULT "[]",
    items TEXT DEFAULT "[]",
    game_id INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Game (id)
);