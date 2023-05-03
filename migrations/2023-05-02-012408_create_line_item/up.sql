-- CREATE TYPE line_item_kind AS ENUM ('standard', 'debt', 'fund');

CREATE TABLE line_items (
    id SERIAL PRIMARY KEY,
    kind line_item_kind NOT NULL,
    name VARCHAR NOT NULL,
    planned REAL NOT NULL,
    balance REAL
);
