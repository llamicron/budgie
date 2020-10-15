table! {
    budget_items (id) {
        id -> Int4,
        item_group_id -> Int4,
        name -> Varchar,
        total -> Float8,
        balance -> Float8,
        fund -> Bool,
        note -> Nullable<Varchar>,
        favorite -> Bool,
    }
}

table! {
    budgets (id) {
        id -> Int4,
        month -> Int4,
        year -> Int4,
    }
}

table! {
    item_groups (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        amount -> Float8,
        merchant -> Nullable<Varchar>,
        note -> Nullable<Varchar>,
        budget_item_id -> Int4,
        created_at -> Timestamptz,
    }
}

joinable!(transactions -> budget_items (budget_item_id));

allow_tables_to_appear_in_same_query!(
    budget_items,
    budgets,
    item_groups,
    transactions,
);
