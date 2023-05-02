CREATE TABLE line_items (
    id INTEGER PRIMARY KEY,
    kind TEXT CHECK(kind in ('standard', 'fund', 'debt')) NOT NULL,
    name VARCHAR,
    planned REAL NOT NULL,
    balance REAL
);
