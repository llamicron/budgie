//! API routes related to budgets

use crate::{db::DbPool, error::BudgieError};
use actix_web::{web, Responder};
use diesel::prelude::*;

pub async fn get_budget(
    path: web::Path<i32>,
    db: web::Data<DbPool>,
) -> Result<impl Responder, BudgieError> {
    use crate::model::Budget;
    use crate::schema::budgets::dsl::*;

    let budget_id = path.into_inner();

    let mut db_conn = db.conns.lock().unwrap().get().unwrap();

    let results = budgets
        .filter(id.eq(budget_id))
        .limit(1)
        .load::<Budget>(&mut db_conn)?;

    if let Some(budget) = results.get(0) {
        return Ok(web::Json(budget.clone()));
    } else {
        return Err(
            BudgieError::ResourceNotFound(format!("Budget with id {budget_id} not found")).into(),
        );
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/budget/{budget_id}").route(web::get().to(get_budget)));
}
