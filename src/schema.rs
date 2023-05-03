// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "line_item_kind"))]
    pub struct LineItemKind;
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
        date -> Timestamp,
    }
}

diesel::joinable!(transactions -> line_items (line_item_id));

diesel::allow_tables_to_appear_in_same_query!(
    line_items,
    transactions,
);
