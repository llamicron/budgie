use diesel::{Queryable, Insertable};

use crate::models::BudgetItem;
use crate::schema::item_groups;

#[derive(Queryable, Identifiable, Debug)]
#[table_name="item_groups"]
pub struct ItemGroup {
    pub id: i32,
    pub name: Option<String>
}

#[derive(Insertable, Debug)]
#[table_name="item_groups"]
pub struct NewItemGroup<'a> {
    pub name: Option<&'a str>
}


// maybe delete this
impl<'a> NewItemGroup<'a> {
    pub fn create(name: Option<&'a str>, child_budget_items: Vec<BudgetItem>) {
        // let new_group_insert = NewItemGroup { name };
        // Insert that ^^

        // Insert new budget items
        unimplemented!();
    }
}
