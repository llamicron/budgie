use super::Result;
use crate::model;
use crate::schema::transactions::{self, dsl::*};
use crate::{db::DbPool, error::BudgieError};

use actix_web::web;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, AsChangeset, Queryable)]
#[diesel(table_name = transactions)]
pub struct NewTransactionPayload {
    pub line_item_id: i32,
    pub is_expense: bool,
    pub amount: f32,
    pub merchant: String,
    pub notes: Option<String>,
    pub date: NaiveDate,
}

pub async fn get_transaction(
    path: web::Path<i32>,
    db: web::Data<DbPool>,
) -> Result<model::Transaction> {
    let trans_id = path.into_inner();

    let mut db_conn = db.conns.lock().unwrap().get().unwrap();

    let results = transactions
        .filter(id.eq(trans_id))
        .limit(1)
        .load::<model::Transaction>(&mut db_conn)?;

    if let Some(trans) = results.get(0) {
        Ok(web::Json(trans.clone()))
    } else {
        Err(BudgieError::ResourceNotFound(format!(
            "Transaction with id {trans_id} not found"
        )))
    }
}

pub async fn new_transaction(
    new_trans: web::Json<NewTransactionPayload>,
    db: web::Data<DbPool>,
) -> Result<model::Transaction> {
    let new = new_trans.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();
    let created = model::Transaction::create(
        conn,
        &new.line_item_id,
        new.is_expense,
        &new.amount,
        &new.merchant,
        new.notes.as_ref().map(|s| s.as_str()),
        new.date,
    )?;
    Ok(web::Json(created))
}

pub async fn update_transaction(
    path: web::Path<i32>,
    update_trans: web::Json<NewTransactionPayload>,
    db: web::Data<DbPool>,
) -> Result<model::Transaction> {
    let new = update_trans.into_inner();
    let trans_id = path.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();

    let result = diesel::update(transactions::table)
        .filter(id.eq(trans_id))
        .set(new)
        .get_result::<model::Transaction>(conn)?;

    Ok(web::Json(result))
}

pub async fn delete_transaction(
    path: web::Path<i32>,
    db: web::Data<DbPool>,
) -> Result<model::Transaction> {
    let trans_id = path.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();

    let result = diesel::delete(transactions.filter(id.eq(trans_id)))
        .get_result::<model::Transaction>(conn)?;
    Ok(web::Json(result))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/transaction/{trans_id}")
            .route(web::get().to(get_transaction))
            .route(web::post().to(update_transaction))
            .route(web::delete().to(delete_transaction)),
    )
    .service(web::resource("/transaction").route(web::post().to(new_transaction)));
}
