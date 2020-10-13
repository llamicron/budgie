use diesel::{Queryable, Insertable};

use crate::schema::budget_items;


#[derive(Queryable, Identifiable)]
pub struct BudgetItem {
    pub id: i32,
    pub name: String,
    pub total: f64,
    pub balance: f64,
    pub fund: bool,
    pub note: Option<String>,
    pub favorite: bool
}

#[derive(Insertable)]
#[table_name="budget_items"]
pub struct NewBudgetItem<'a> {
    pub name: &'a str,
    pub total: f64,
    pub balance: f64,
    pub fund: bool,
    pub note: Option<&'a str>,
    pub favorite: bool
}
