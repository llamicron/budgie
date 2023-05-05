//! API routes related to budgets

use super::Result;
use crate::model;
use crate::schema::budgets::{self, dsl::*};
use crate::schema::line_item_groups;
use crate::{db::DbPool, error::BudgieError};

use actix_web::web;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Deserialize;

/// This mirrors NewBudget from the model, used to insert a new row into the db.
/// It uses lifetimes in a way that actix-web can't handle so we have to have 2 unfortunately
#[derive(Deserialize, AsChangeset, Queryable)]
#[diesel(table_name = budgets)]
pub struct NewBudgetPayload {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub name: String,
    pub notes: Option<String>,
}

/// Finds a budget by ID if it exists
pub async fn get_budget(path: web::Path<i32>, db: web::Data<DbPool>) -> Result<model::Budget> {
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

pub async fn new_budget(
    new_budget: web::Json<NewBudgetPayload>,
    db: web::Data<DbPool>,
) -> Result<model::Budget> {
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
) -> Result<model::Budget> {
    let new = update_budget.into_inner();
    let budget_id = path.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();

    let result = diesel::update(budgets::table)
        .filter(id.eq(budget_id))
        .set(new)
        .get_result::<model::Budget>(conn)?;

    Ok(web::Json(result))
}

pub async fn delete_budget(path: web::Path<i32>, db: web::Data<DbPool>) -> Result<model::Budget> {
    let budget_id = path.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();

    let result =
        diesel::delete(budgets.filter(id.eq(budget_id))).get_result::<model::Budget>(conn)?;
    Ok(web::Json(result))
}

pub async fn get_line_item_groups(
    path: web::Path<i32>,
    db: web::Data<DbPool>,
) -> Result<Vec<model::LineItemGroup>> {
    use crate::schema::line_item_groups::dsl::*;
    let bud_id = path.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();

    let groups = line_item_groups
        .filter(budget_id.eq(bud_id))
        .get_results(conn)?;

    Ok(web::Json(groups))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/budget/{budget_id}")
            .route(web::get().to(get_budget))
            .route(web::post().to(update_budget))
            .route(web::delete().to(delete_budget)),
    )
    .service(web::resource("/budget").route(web::post().to(new_budget)))
    .service(
        web::resource("/budget/{budget_id}/line_item_groups")
            .route(web::get().to(get_line_item_groups)),
    );
}
