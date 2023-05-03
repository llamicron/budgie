use crate::error::Result;
use crate::schema::line_item_groups;
use diesel::prelude::*;

#[derive(Debug, Queryable)]
#[diesel(table_name = line_item_groups)]
pub struct LineItemGroup {
    pub id: i32,
    pub name: String,
}

impl LineItemGroup {
    pub fn create(db: &mut PgConnection, name: &str) -> Result<LineItemGroup> {
        let new_group = NewLineItemGroup { name };

        diesel::insert_into(line_item_groups::table)
            .values(&new_group)
            .get_result(db)
            .map_err(|e| e.into())
    }
}

impl Default for LineItemGroup {
    fn default() -> Self {
        let db = &mut crate::db::connect().unwrap();
        LineItemGroup::create(db, "New Group").unwrap()
    }
}

#[derive(Insertable)]
#[diesel(table_name = line_item_groups)]
pub struct NewLineItemGroup<'a> {
    name: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db, schema::line_item_groups::dsl::line_item_groups};

    fn nuke(db: &mut PgConnection) {
        diesel::delete(line_item_groups).execute(db).unwrap();
    }

    #[test]
    fn test_insert_line_item_group() {
        let db = &mut db::connect().unwrap();
        nuke(db);

        let group = LineItemGroup::default();

        assert_eq!(group.name, "New Group");

        nuke(db);
    }
}
