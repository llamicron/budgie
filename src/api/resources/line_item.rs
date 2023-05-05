use super::Result;
use crate::model;
use crate::schema::line_items::{self, dsl::*};
use crate::{db::DbPool, error::BudgieError};

use actix_web::web;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, AsChangeset, Queryable)]
#[diesel(table_name = line_items)]
pub struct NewLineItemPayload {
    pub kind: model::LineItemKind,
    pub name: String,
    pub planned: f32,
    pub balance: Option<f32>,
    pub group_id: i32,
}

pub async fn get_line_item(path: web::Path<i32>, db: web::Data<DbPool>) -> Result<model::LineItem> {
    let li_id = path.into_inner();

    let mut db_conn = db.conns.lock().unwrap().get().unwrap();

    let results = line_items
        .filter(id.eq(li_id))
        .limit(1)
        .load::<model::LineItem>(&mut db_conn)?;

    if let Some(line_item) = results.get(0) {
        Ok(web::Json(line_item.clone()))
    } else {
        Err(BudgieError::ResourceNotFound(format!(
            "Line Item with id {li_id} not found"
        )))
    }
}

pub async fn new_line_item(
    new_line_item: web::Json<NewLineItemPayload>,
    db: web::Data<DbPool>,
) -> Result<model::LineItem> {
    let new = new_line_item.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();
    let created = model::LineItem::create(
        conn,
        &new.kind,
        &new.name,
        &new.planned,
        new.balance.as_ref(),
        new.group_id,
    )?;
    Ok(web::Json(created))
}

pub async fn update_line_item(
    path: web::Path<i32>,
    update_line_item: web::Json<NewLineItemPayload>,
    db: web::Data<DbPool>,
) -> Result<model::LineItem> {
    let new = update_line_item.into_inner();
    let li_id = path.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();

    let result = diesel::update(line_items::table)
        .filter(id.eq(li_id))
        .set(new)
        .get_result::<model::LineItem>(conn)?;

    Ok(web::Json(result))
}

pub async fn delete_line_item(
    path: web::Path<i32>,
    db: web::Data<DbPool>,
) -> Result<model::LineItem> {
    let li_id = path.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();

    let result =
        diesel::delete(line_items.filter(id.eq(li_id))).get_result::<model::LineItem>(conn)?;
    Ok(web::Json(result))
}

pub async fn get_transactions(
    path: web::Path<i32>,
    db: web::Data<DbPool>,
) -> Result<Vec<model::Transaction>> {
    use crate::schema::transactions::dsl::*;
    let li_id = path.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();

    let trans = transactions
        .filter(line_item_id.eq(li_id))
        .get_results(conn)?;

    Ok(web::Json(trans))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/line_item/{line_item}")
            .route(web::get().to(get_line_item))
            .route(web::post().to(update_line_item))
            .route(web::delete().to(delete_line_item)),
    )
    .service(web::resource("/line_item").route(web::post().to(new_line_item)))
    .service(
        web::resource("/line_item/{line_item_id}/transactions")
            .route(web::get().to(get_transactions)),
    );
}
