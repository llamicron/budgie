CREATE TABLE budget_items (
    id INT PRIMARY KEY,
    name VARCHAR NOT NULL,
    total DOUBLE PRECISION NOT NULL,
    balance DOUBLE PRECISION NOT NULL DEFAULT 0,
    fund BOOLEAN NOT NULL DEFAULT FALSE,
    note VARCHAR,
    favorite BOOLEAN NOT NULL DEFAULT FALSE
)
