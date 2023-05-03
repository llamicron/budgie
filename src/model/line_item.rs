//! Line items are the building blocks of a budget.
//! Every line item has a type, Standard, Fund, or Debt.
//!
//! Standard line items have a name and "planned" amount. This is like the total
//! amount for the budget. To get the current balance of the budget item, we go
//! through transactions tied to this line item and subtract their sum total from
//! the planned amount. The line item only stores the planned amount.
//!
//! Additionally, Debt and Funds are like opposites of each other. They have a balance
//! and the planned amount for the line item contributes to that balance. A fund may have
//! a starting balance of $500, and when it's used in a budget that sets is "planned" value
//! to $75, it now has $575 in it.

use crate::error::Result;
use crate::schema::line_items;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

use super::LineItemGroup;

#[derive(Debug, DbEnum, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::LineItemKind"]
pub enum LineItemKind {
    Standard,
    Fund,
    Debt,
}

#[derive(Debug, Queryable, Identifiable, PartialEq)]
#[diesel(table_name = line_items)]
pub struct LineItem {
    pub id: i32,
    pub kind: LineItemKind,
    pub name: String,
    /// The planned amount for a certain budget.
    /// This is like a total.
    pub planned: f32,
    /// Balance is used for debts and funds, to carry over
    pub balance: Option<f32>,
    pub group_id: i32,
}

impl LineItem {
    /// Inserts a new line item into the DB
    pub fn create(
        conn: &mut PgConnection,
        kind: &LineItemKind,
        name: &str,
        planned: &f32,
        balance: Option<&f32>,
        group_id: i32,
    ) -> Result<LineItem> {
        let new_line_item = NewLineItem {
            kind,
            name,
            planned,
            balance,
            group_id,
        };

        diesel::insert_into(line_items::table)
            .values(&new_line_item)
            .get_result(conn)
            .map_err(|e| e.into())
    }

    /// Moves a line item from it's current group to a new one
    pub fn move_to_group(&mut self, db: &mut PgConnection, group: &LineItemGroup) -> Result<usize> {
        use crate::schema::line_items::dsl::*;
        let result = diesel::update(line_items)
            .filter(id.eq(self.id))
            .set(group_id.eq(group.id))
            .execute(db)
            .map_err(|e| e.into());

        self.group_id = group.id;
        result
    }
}

#[derive(Insertable)]
#[diesel(table_name = line_items)]
struct NewLineItem<'a> {
    pub kind: &'a LineItemKind,
    pub name: &'a str,
    pub planned: &'a f32,
    pub balance: Option<&'a f32>,
    pub group_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::model::LineItemGroup;
    use crate::schema::line_items;

    #[test]
    fn test_insert_new_line_item() {
        let db = &mut db::connect().unwrap();
        db::nuke(db);

        let group = LineItemGroup::default();

        let old_count = line_items::table.count().first::<i64>(db).unwrap();
        assert_eq!(old_count, 0);
        LineItem::create(db, &LineItemKind::Standard, "Gas", &120.0, None, group.id).unwrap();
        let new_count = line_items::table.count().first::<i64>(db).unwrap();
        assert_eq!(new_count, 1);

        let li_it = line_items::table.first::<LineItem>(db).unwrap();
        assert_eq!(li_it.name, "Gas");

        db::nuke(db);
    }

    #[test]
    fn test_move_group() {
        let db = &mut db::connect().unwrap();
        db::nuke(db);
        let group1 = LineItemGroup::default();
        let group2 = LineItemGroup::default();

        assert_ne!(group1.id, group2.id);

        let mut line_item =
            LineItem::create(db, &LineItemKind::Standard, "Gas", &120.0, None, group1.id).unwrap();

        assert_eq!(line_item.group_id, group1.id);
        line_item.move_to_group(db, &group2).unwrap();
        assert_eq!(line_item.group_id, group2.id);
        db::nuke(db);
    }
}
