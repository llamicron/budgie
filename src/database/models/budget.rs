use diesel::{Insertable, Identifiable};

use crate::database::schema::budgets;

#[derive(Debug, Queryable, Insertable, Identifiable)]
#[table_name="budgets"]
pub struct Budget {
    pub id: i32,
    pub month: i32,
    pub year: i32
}


#[derive(Insertable, Debug)]
#[table_name="budgets"]
pub struct NewBudget {
    pub month: i32,
    pub year: i32
}
