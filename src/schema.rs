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
