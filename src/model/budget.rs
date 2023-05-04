use crate::error::Result;
use crate::schema::budgets;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Queryable, Identifiable)]
#[diesel(table_name = budgets)]
pub struct Budget {
    pub id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub name: String,
    pub notes: Option<String>,
}

impl Budget {
    pub fn create(
        db: &mut PgConnection,
        start_date: NaiveDate,
        end_date: NaiveDate,
        name: &str,
        notes: Option<&str>,
    ) -> Result<Budget> {
        let new_budget = NewBudget {
            start_date,
            end_date,
            name,
            notes,
        };

        diesel::insert_into(budgets::table)
            .values(&new_budget)
            .get_result(db)
            .map_err(|e| e.into())
    }

    pub fn create_from(db: &mut PgConnection, new_budget: NewBudget) -> Result<Budget> {
        diesel::insert_into(budgets::table)
            .values(&new_budget)
            .get_result(db)
            .map_err(|e| e.into())
    }
}

#[derive(Insertable)]
#[diesel(table_name = budgets)]
pub struct NewBudget<'a> {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub name: &'a str,
    pub notes: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_insert_budget() {
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

        assert_eq!(budget.name, "May Budget");
        db::nuke(db);
    }
}
