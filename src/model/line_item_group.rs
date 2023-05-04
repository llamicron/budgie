use crate::error::Result;
use crate::schema::line_item_groups;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Queryable)]
#[diesel(table_name = line_item_groups)]
pub struct LineItemGroup {
    pub id: i32,
    pub name: String,
    pub budget_id: i32,
}

impl LineItemGroup {
    pub fn create(db: &mut PgConnection, budget_id: i32, name: &str) -> Result<LineItemGroup> {
        let new_group = NewLineItemGroup { budget_id, name };

        diesel::insert_into(line_item_groups::table)
            .values(&new_group)
            .get_result(db)
            .map_err(|e| e.into())
    }
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = line_item_groups)]
pub struct NewLineItemGroup<'a> {
    budget_id: i32,
    name: &'a str,
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;
    use crate::{db, model::Budget};

    #[test]
    fn test_insert_line_item_group() {
        let db = &mut db::connect().unwrap();
        db::nuke(db);

        let budget = Budget::create(
            db,
            NaiveDate::from_ymd_opt(2023, 5, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 5, 31).unwrap(),
            "May Budget",
            None,
        )
        .unwrap();

        let group = LineItemGroup::create(db, budget.id, "New Group").unwrap();

        assert_eq!(group.name, "New Group");

        db::nuke(db);
    }
}
