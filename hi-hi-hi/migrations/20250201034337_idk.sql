CREATE TABLE IF NOT EXISTS client_data (
    client_data_id INTEGER PRIMARY KEY AUTOINCREMENT,
    id INTEGER NOT NULL,
    r#type INTEGER NOT NULL,
    data BLOB NOT NULL
);
