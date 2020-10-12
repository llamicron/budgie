use crate::transaction::Transaction;

#[derive(Debug)]
pub struct BudgetGroup(Vec<BudgetItem>);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BudgetItemType {
    /// A budget item that starts at 0.
    /// You're meant to put Income transactions into these
    Fund,
    /// A budget item that starts to the total set.
    /// You're meant to take Expense transactions from these
    Item
}


#[derive(Debug)]
pub struct BudgetItem {
    id: String,
    name: String,
    total: f32,
    balance: f32,
    item_type: BudgetItemType,
    note: Option<String>,
    transactions: Vec<Transaction>,
    favorite: bool
}

impl BudgetItem {
    /// Creates a new budget item with a name and total.
    /// By default, this is a regular budget item (meant to take money out of).
    /// It can be set as a Fund, which is the opposite.
    pub fn new(name: String, total: f32) -> BudgetItem {
        BudgetItem {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            total,
            balance: total,
            item_type: BudgetItemType::Item,
            note: None,
            transactions: Vec::new(),
            favorite: false
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    /// Gets the items name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the items total
    pub fn total(&self) -> f32 {
        self.total
    }


    /// Gets the items current balance (total +- all transactions)
    pub fn balance(&self) -> f32 {
        self.balance
    }

    /// Gets the budget items type (Item or Fund)
    pub fn item_type(&self) -> BudgetItemType {
        self.item_type
    }

    /// Sets the items total
    pub fn set_total(&mut self, new_total: f32) {
        self.total = new_total;
    }

    /// Sets the items type
    pub fn set_type(&mut self, new_type: BudgetItemType) {
        self.item_type = new_type
    }

    /// Sets the items favorite flag
    pub fn set_favorite(&mut self, favorite: bool) {
        self.favorite = favorite
    }

    /// Returns true if this item is a favorite
    pub fn is_favorite(&self) -> bool {
        self.favorite
    }

    pub fn add_transaction(&mut self, trans: Transaction) {
        self.balance += trans.amount();
        self.transactions.push(trans);
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_budget_item() {
        let mut item = BudgetItem::new("Gas".to_string(), 150.0);
        assert!(item.id().len() > 5);
        assert_eq!(item.name(), "Gas");
        assert_eq!(item.total(), 150.0);
        assert_eq!(item.balance(), 150.0);

        item.set_total(135.0);
        assert_eq!(item.total(), 135.0);

        item.set_favorite(true);
        assert!(item.is_favorite());

        item.set_type(BudgetItemType::Fund);
        assert_eq!(item.item_type(), BudgetItemType::Fund);
    }

    #[test]
    fn test_add_transaction() {
        let mut item = BudgetItem::new("Gas".to_string(), 150.0);
        assert_eq!(item.balance(), 150.0);

        let trans = Transaction::new(30.0);
        item.add_transaction(trans);

        assert_eq!(item.balance(), 120.0);
    }
}
