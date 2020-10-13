extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate uuid;
#[macro_use]
extern crate diesel;
extern crate dotenv;



mod database;
mod schema;
mod models;

fn main() {
    use schema::budget_items::dsl::*;
    use schema::transactions::dsl::*;
    use crate::diesel::prelude::*;
    use models::*;

    let conn = database::establish_connection();

    // let new_item = NewBudgetItem {
    //     name: "Gas",
    //     total: 150.0,
    //     balance: 130.6,
    //     fund: false,
    //     note: Some("hello there!"),
    //     favorite: true
    // };

    // diesel::insert_into(budget_items::table)
    //     .values(&new_item)
    //     .get_results::<BudgetItem>(&conn)
    //     .expect("Something went wrong");

    let budget_item = budget_items.find(1).first::<BudgetItem>(&conn).unwrap();
    let trans = Transaction::belonging_to(&budget_item).load::<Transaction>(&conn).expect("Uh oh");
    
    // let new_trans = NewTransaction {
    //     amount: 145.0,
    //     merchant: Some("Exxon"),
    //     note: None,
    //     budget_item_id: Some(budget_item.id)
    // };

    // diesel::insert_into(schema::transactions::table)
    //     .values(&new_trans)
    //     .get_results::<Transaction>(&conn)
    //     .expect("Something went wrong");

}
