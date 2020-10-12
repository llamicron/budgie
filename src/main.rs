extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate uuid;

mod transaction;
mod budget_item;

fn main() {
    let mut item = budget_item::BudgetItem::new("Gas".to_string(), 150.0);
    let mut trans = transaction::Transaction::new(35.0);
    trans.set_merchant("Exxon".to_string());
    let mut trans2 = transaction::Transaction::new(35.0);
    trans2.set_merchant("QuikTrip".to_string());
    item.add_transaction(trans);
    item.add_transaction(trans2);

    println!("{:#x?}", item);

    let mut trans3 = transaction::Transaction::new(25.0);
    trans3.set_trans_type(transaction::TransactionType::Income);
    item.add_transaction(trans3);

}
