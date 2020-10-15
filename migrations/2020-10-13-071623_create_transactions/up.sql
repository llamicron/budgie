CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    amount DOUBLE PRECISION NOT NULL,
    merchant VARCHAR,
    note VARCHAR,
    budget_item_id SERIAL
)
