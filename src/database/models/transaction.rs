use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};

use crate::database::schema::transactions;
use crate::database::models::BudgetItem;

#[derive(Identifiable, Queryable, Associations, Debug, Deserialize, Serialize)]
#[table_name="transactions"]
#[belongs_to(BudgetItem)]
pub struct Transaction {
    pub id: i32,
    pub amount: f64,
    pub merchant: Option<String>,
    pub note: Option<String>,
    pub budget_item_id: i32
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name="transactions"]
pub struct NewTransaction<'a> {
    pub amount: f64,
    pub merchant: Option<&'a str>,
    pub note: Option<&'a str>,
    pub budget_item_id: Option<i32>
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_read_transactions() {
        use crate::database::schema::transactions::dsl::*;
        use diesel::prelude::*;
        let conn = crate::database::establish_connection();
        
        let new_trans = NewTransaction {
            amount: 45.5,
            merchant: None,
            note: None,
            budget_item_id: None
        };

        let found = diesel::insert_into(transactions).values(&new_trans).get_result::<Transaction>(&conn).unwrap();
        assert_eq!(found.amount, 45.5);
    }
}
