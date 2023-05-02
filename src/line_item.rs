#![allow(dead_code, unused)]

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

use crate::schema::line_items;
use crate::ID;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum)]
pub enum LineItemKind {
    Standard,
    Fund,
    Debt,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = line_items)]
pub struct LineItem {
    pub id: ID,
    pub kind: LineItemKind,
    pub name: String,
    /// The planned amount for a certain budget.
    /// This is like a total.
    pub planned: f32,
    /// Balance is used for debts and funds, to carry over
    pub balance: Option<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
