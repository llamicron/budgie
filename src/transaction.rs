use chrono::{DateTime, Local};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TransactionType {
    Expense,
    Income
}

#[derive(Debug)]
pub struct Transaction {
    amount: f32,
    merchant: Option<String>,
    // TODO: Replace this with custom ID type?
    item_id: Option<String>,
    note: Option<String>,
    trans_type: TransactionType,
    date: DateTime<Local>
}

impl Transaction {
    pub fn new(amount: f32) -> Transaction {
        Transaction {
            amount,
            merchant: None,
            item_id: None,
            note: None,
            trans_type: TransactionType::Expense,
            date: Local::now()
        }
    }

    pub fn note(&self) -> Option<&String> {
        self.note.as_ref()
    }

    pub fn set_note(&mut self, note: String) {
        self.note = Some(note);
    }

    pub fn merchant(&mut self) -> Option<&String> {
        self.merchant.as_ref()
    }

    pub fn set_merchant(&mut self, merchant: String) {
        self.merchant = Some(merchant);
    }

    pub fn trans_type(&self) -> TransactionType {
        self.trans_type
    }

    pub fn set_trans_type(&mut self, trans_type: TransactionType) {
        self.trans_type = trans_type;
    }

    pub fn item_id(&self) -> Option<&String> {
        self.item_id.as_ref()
    }

    pub fn set_item_id(&mut self, item_id: String) {
        self.item_id = Some(item_id);
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_new_transaction() {
        // All that's required is an amount
        let trans = Transaction::new(4.89);
        assert_eq!(trans.amount, 4.89);
        assert_eq!(trans.trans_type, TransactionType::Expense);
    }
    
    #[test]
    fn test_transaction_data() {
        let mut trans = Transaction::new(4.89);
        assert_eq!(trans.amount, 4.89);
        assert_eq!(trans.trans_type, TransactionType::Expense);

        trans.set_note("a note".to_string());
        assert!(trans.note.is_some());
        assert_eq!(trans.note().unwrap(), "a note");

        trans.set_merchant("GenericGasStation".to_string());
        assert_eq!(trans.merchant().unwrap(), "GenericGasStation");

        trans.set_trans_type(TransactionType::Income);
        assert_eq!(trans.trans_type(), TransactionType::Income);

        trans.set_item_id("123456".to_string());
        assert_eq!(trans.item_id().unwrap(), "123456");
    }
}
