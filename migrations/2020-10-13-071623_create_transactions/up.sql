CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    amount DOUBLE PRECISION NOT NULL,
    merchant VARCHAR,
    note VARCHAR,
    budget_item_id SERIAL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_budget_item
        FOREIGN KEY(budget_item_id)
            REFERENCES budget_items(id)
)
