extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate uuid;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod database;

fn main() {
    // use diesel::prelude::*;

    // let conn = database::establish_connection();

    // // Insert a group
    // let new_group = models::NewItemGroup { name: Some("Transportation") };
    // let group = diesel::insert_into(schema::item_groups::table)
    //     .values(&new_group)
    //     .get_result::<models::ItemGroup>(&conn)
    //     .expect("Couldn't return item group");

    // println!("{:#?}", group);

    // // Insert two items
    // let new_item = models::NewBudgetItem {
    //     item_group_id: Some(group.id),
    //     name: "Gas",
    //     total: 150.0,
    //     balance: 132.97,
    //     fund: false,
    //     note: None,
    //     favorite: false
    // };

    // let new_item2 = models::NewBudgetItem {
    //     item_group_id: Some(group.id),
    //     name: "Public Transport",
    //     total: 40.0,
    //     balance: 34.0,
    //     fund: false,
    //     note: Some("For riding the bus and such"),
    //     favorite: true
    // };

    // let items = diesel::insert_into(schema::budget_items::table)
    //     .values(vec![&new_item, &new_item2])
    //     .get_results::<models::BudgetItem>(&conn)
    //     .expect("Couldn't get items back");
    
    // println!("{:#?}", items);

    // let found_items = models::BudgetItem::belonging_to(&group).get_results::<models::BudgetItem>(&conn).unwrap();
    // println!("{:#?}", found_items);

    // diesel::delete(schema::item_groups::table).execute(&conn).ok();
    // diesel::delete(schema::budget_items::table).execute(&conn).ok();
    // diesel::delete(schema::transactions::table).execute(&conn).ok();
}
