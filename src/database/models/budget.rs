use diesel::{Insertable, Identifiable};
use serde::{Deserialize, Serialize};

use crate::database::schema::budgets;

#[derive(Debug, Queryable, Insertable, Identifiable, Deserialize, Serialize)]
#[table_name="budgets"]
pub struct Budget {
    pub id: i32,
    pub month: i32,
    pub year: i32
}


#[derive(Insertable, Debug, Deserialize, Serialize)]
#[table_name="budgets"]
pub struct NewBudget {
    pub month: i32,
    pub year: i32
}
