use diesel::{Queryable, Insertable};
use diesel::data_types::PgTimestamp;

use crate::schema::transactions;
use crate::models::BudgetItem;

#[derive(Queryable, Associations)]
#[belongs_to(BudgetItem, foreign_key = "id")]
#[table_name="transactions"]
pub struct Transaction {
    pub id: i32,
    pub amount: f64,
    pub merchant: Option<String>,
    pub note: Option<String>,
    pub budget_item_id: i32,
    pub created_at: PgTimestamp
}

#[derive(Insertable)]
#[table_name="transactions"]
pub struct NewTransaction<'a> {
    pub amount: f64,
    pub merchant: Option<&'a str>,
    pub note: Option<&'a str>,
    pub budget_item_id: Option<i32>
}
