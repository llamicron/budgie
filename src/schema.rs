table! {
    transactions (id) {
        id -> Int4,
        amount -> Float8,
        merchant -> Nullable<Varchar>,
        note -> Nullable<Varchar>,
    }
}
