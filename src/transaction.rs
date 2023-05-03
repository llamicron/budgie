use crate::error::{BudgieError, Result};
use crate::schema::transactions;
use diesel::prelude::*;

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub line_item_id: i32,
    pub is_expense: bool,
    pub amount: f32,
    pub merchant: String,
    pub notes: Option<String>,
    pub date: chrono::NaiveDateTime,
}

impl Transaction {
    pub fn create(
        db: &mut PgConnection,
        line_item_id: &i32,
        is_expense: bool,
        amount: &f32,
        merchant: &str,
        notes: Option<&str>,
        date: chrono::NaiveDateTime,
    ) -> Result<Self> {
        let new_trans = NewTransaction {
            line_item_id,
            is_expense,
            amount,
            merchant,
            notes,
            date,
        };

        diesel::insert_into(transactions::table)
            .values(&new_trans)
            .get_result(db)
            .map_err(|e| BudgieError::from(e))
    }
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    line_item_id: &'a i32,
    is_expense: bool,
    amount: &'a f32,
    merchant: &'a str,
    notes: Option<&'a str>,
    date: chrono::NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transaction() {}
}
