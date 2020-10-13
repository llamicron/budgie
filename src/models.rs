use diesel::Queryable;

#[derive(Queryable)]
pub struct Transaction {
    pub id: i32,
    pub amount: f64,
    pub merchant: Option<String>,
    pub note: Option<String>
}
