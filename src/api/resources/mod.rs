pub mod budget;
pub mod line_item_groups;

type Result<T> = std::result::Result<actix_web::web::Json<T>, crate::error::BudgieError>;
