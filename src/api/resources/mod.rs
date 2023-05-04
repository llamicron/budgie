pub mod budget;
pub mod line_item;
pub mod line_item_group;
pub mod transactions;

type Result<T> = std::result::Result<actix_web::web::Json<T>, crate::error::BudgieError>;
