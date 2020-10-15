use diesel::{Queryable, Insertable};

use crate::database::models::Budget;
use crate::database::schema::item_groups;

#[derive(Queryable, Identifiable, Debug, Associations)]
#[table_name="item_groups"]
#[belongs_to(Budget)]
pub struct ItemGroup {
    pub id: i32,
    pub budget_id: i32,
    pub name: Option<String>
}

#[derive(Insertable, Debug)]
#[table_name="item_groups"]
pub struct NewItemGroup<'a> {
    pub name: Option<&'a str>,
    pub budget_id: i32
}
