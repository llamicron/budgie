CREATE TABLE budgets (
    id SERIAL PRIMARY KEY,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    name TEXT NOT NULL,
    notes TEXT
);
