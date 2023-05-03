CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    line_item_id INTEGER NOT NULL REFERENCES line_items(id),
    is_expense BOOLEAN NOT NULL,
    amount REAL NOT NULL,
    merchant TEXT NOT NULL,
    notes TEXT,
    date TIMESTAMP NOT NULL
);
