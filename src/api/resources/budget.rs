//! API routes related to budgets

use crate::model;
use crate::schema::budgets::{self, dsl::*};
use crate::{db::DbPool, error::BudgieError};
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Deserialize;

/// Finds a budget by ID if it exists
pub async fn get_budget(
    path: web::Path<i32>,
    db: web::Data<DbPool>,
) -> Result<impl Responder, BudgieError> {
    let budget_id = path.into_inner();

    let mut db_conn = db.conns.lock().unwrap().get().unwrap();

    let results = budgets
        .filter(id.eq(budget_id))
        .limit(1)
        .load::<model::Budget>(&mut db_conn)?;

    if let Some(budget) = results.get(0) {
        Ok(web::Json(budget.clone()))
    } else {
        Err(BudgieError::ResourceNotFound(format!(
            "Budget with id {budget_id} not found"
        )))
    }
}

/// This mirrors NewBudget from the model, used to insert a new row into the db.
/// It uses lifetimes in a way that actix-web can't handle so we have to have 2 unfortunately.
#[derive(Deserialize, AsChangeset, Queryable)]
#[diesel(table_name = budgets)]
pub struct NewBudgetPayload {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub name: String,
    pub notes: Option<String>,
}

pub async fn new_budget(
    new_budget: web::Json<NewBudgetPayload>,
    db: web::Data<DbPool>,
) -> Result<web::Json<model::Budget>, BudgieError> {
    let new = new_budget.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();
    let created = model::Budget::create(
        conn,
        new.start_date,
        new.end_date,
        &new.name,
        new.notes.as_ref().map(|s| s.as_str()),
    )?;
    Ok(web::Json(created))
}

pub async fn update_budget(
    path: web::Path<i32>,
    update_budget: web::Json<NewBudgetPayload>,
    db: web::Data<DbPool>,
) -> Result<web::Json<model::Budget>, BudgieError> {
    let new = update_budget.into_inner();
    let budget_id = path.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();

    let result = diesel::update(budgets::table)
        .filter(id.eq(budget_id))
        .set(new)
        .get_result::<model::Budget>(conn)?;

    Ok(web::Json(result))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/budget/{budget_id}")
            .route(web::get().to(get_budget))
            .route(web::post().to(update_budget)),
    )
    .service(web::resource("/budget").route(web::post().to(new_budget)));
}
