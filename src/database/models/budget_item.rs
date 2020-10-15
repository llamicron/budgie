use diesel::{Queryable, Insertable};

use crate::database::schema::budget_items;
use crate::database::models::ItemGroup;


#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(ItemGroup)]
#[table_name="budget_items"]
pub struct BudgetItem {
    pub id: i32,
    pub item_group_id: i32,
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
    pub item_group_id: Option<i32>,
    pub total: f64,
    pub balance: f64,
    pub fund: bool,
    pub note: Option<&'a str>,
    pub favorite: bool
}
