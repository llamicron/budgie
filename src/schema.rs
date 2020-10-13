table! {
    budget_items (id) {
        id -> Int4,
        name -> Varchar,
        total -> Float8,
        balance -> Float8,
        fund -> Bool,
        note -> Nullable<Varchar>,
        favorite -> Bool,
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
    transactions,
);
