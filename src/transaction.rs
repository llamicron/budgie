// This will probably get deleted soon


// use chrono::{DateTime, Local};

// /// The possible types of transaction
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub enum TransactionType {
//     Expense,
//     Income
// }

// /// One basic transaction. Either income or expense.
// /// 
// /// Transactions need an amount and type, but other fields are optional.
// #[derive(Debug)]
// pub struct Transaction {
//     /// A unique transaction ID
//     id: String,
//     /// The currency amount of the transaction
//     amount: f32,
//     /// The name of the merchant where the money was spent
//     merchant: Option<String>,
//     // TODO: Replace this with custom ID type?
//     /// The ID of the budget item this belongs to.
//     /// An empty item id means the transaction is "floating",
//     /// meaning it isn't (yet) associated with an item.
//     item_id: Option<String>,
//     /// Optional notes about the transaction
//     note: Option<String>,
//     /// Expense or Income
//     trans_type: TransactionType,
//     /// The time of the transaction. This is automatically set to
//     /// the current time when the Transaction is created, in local time.
//     /// This is not correct, it will need to be changed later. I hate timezones.
//     date: DateTime<Local>
// }

// impl Transaction {
//     /// Creates a new transaction with the given amount, as an Expense
//     pub fn new(amount: f32) -> Transaction {
//         Transaction {
//             id: uuid::Uuid::new_v4().to_string(),
//             amount,
//             merchant: None,
//             item_id: None,
//             note: None,
//             trans_type: TransactionType::Expense,
//             date: Local::now()
//         }
//     }

//     /// Returns -amount if this is an expense, +amount if it's income
//     pub fn amount(&self) -> f32 {
//         match self.trans_type {
//             TransactionType::Expense => - self.amount,
//             TransactionType::Income => self.amount
//         }
//     }

//     /// Gets the transaction ID
//     pub fn id(&self) -> &str {
//         &self.id
//     }

//     /// Gets the attached note
//     pub fn note(&self) -> Option<&String> {
//         self.note.as_ref()
//     }

//     /// Sets the attached note
//     pub fn set_note(&mut self, note: String) {
//         self.note = Some(note);
//     }

//     /// Gets the attached merchant
//     pub fn merchant(&mut self) -> Option<&String> {
//         self.merchant.as_ref()
//     }

//     /// Sets the attached merchant
//     pub fn set_merchant(&mut self, merchant: String) {
//         self.merchant = Some(merchant);
//     }

//     /// Gets the transaction type. Expense or Income
//     pub fn trans_type(&self) -> TransactionType {
//         self.trans_type
//     }

//     /// Sets the transaction type
//     pub fn set_trans_type(&mut self, trans_type: TransactionType) {
//         self.trans_type = trans_type;
//     }

//     /// Gets the associated budget item ID
//     pub fn item_id(&self) -> Option<&String> {
//         self.item_id.as_ref()
//     }

//     /// Sets the associated budget item ID
//     pub fn set_item_id(&mut self, item_id: String) {
//         self.item_id = Some(item_id);
//     }
// }



// #[cfg(test)]
// mod tests {
//     use super::*;
    

//     #[test]
//     fn test_new_transaction() {
//         // All that's required is an amount
//         let trans = Transaction::new(4.89);
//         assert_eq!(trans.amount, 4.89);
//         assert_eq!(trans.trans_type, TransactionType::Expense);
//     }
    
//     #[test]
//     fn test_transaction_data() {
//         let mut trans = Transaction::new(4.89);
//         assert_eq!(trans.amount, 4.89);
//         assert_eq!(trans.trans_type, TransactionType::Expense);

//         trans.set_note("a note".to_string());
//         assert!(trans.note.is_some());
//         assert_eq!(trans.note().unwrap(), "a note");

//         trans.set_merchant("GenericGasStation".to_string());
//         assert_eq!(trans.merchant().unwrap(), "GenericGasStation");

//         trans.set_trans_type(TransactionType::Income);
//         assert_eq!(trans.trans_type(), TransactionType::Income);

//         trans.set_item_id("123456".to_string());
//         assert_eq!(trans.item_id().unwrap(), "123456");
//     }
// }
