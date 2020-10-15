use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

// Delete alls records in the db
#[allow(unused)]
pub fn wipe() {
    use crate::schema::{budgets, budget_items, item_groups, transactions};
    let conn = establish_connection();

    diesel::delete(budgets::table).execute(&conn).ok();
    diesel::delete(budget_items::table).execute(&conn).ok();
    diesel::delete(item_groups::table).execute(&conn).ok();
    diesel::delete(transactions::table).execute(&conn).ok();
}
