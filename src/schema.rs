// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "line_item_kind"))]
    pub struct LineItemKind;
}

diesel::table! {
    budgets (id) {
        id -> Int4,
        start_date -> Date,
        end_date -> Date,
        name -> Text,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    line_item_groups (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LineItemKind;

    line_items (id) {
        id -> Int4,
        kind -> LineItemKind,
        name -> Varchar,
        planned -> Float4,
        balance -> Nullable<Float4>,
        group_id -> Int4,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        line_item_id -> Int4,
        is_expense -> Bool,
        amount -> Float4,
        merchant -> Text,
        notes -> Nullable<Text>,
        date -> Date,
    }
}

diesel::joinable!(line_items -> line_item_groups (group_id));
diesel::joinable!(transactions -> line_items (line_item_id));

diesel::allow_tables_to_appear_in_same_query!(
    budgets,
    line_item_groups,
    line_items,
    transactions,
);
