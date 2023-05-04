ALTER TABLE line_item_groups
ADD COLUMN budget_id INTEGER NOT NULL REFERENCES budgets(id);
