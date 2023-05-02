// @generated automatically by Diesel CLI.

diesel::table! {
    line_items (id) {
        id -> Nullable<Integer>,
        kind -> crate::line_item::LineItemKindMapping,
        name -> Nullable<Text>,
        planned -> Float,
        balance -> Nullable<Float>,
    }
}
