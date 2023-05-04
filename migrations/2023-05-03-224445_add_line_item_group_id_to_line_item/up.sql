ALTER TABLE line_items
ADD COLUMN group_id INTEGER NOT NULL REFERENCES line_item_groups(id);

