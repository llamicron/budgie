use crate::model;
use crate::schema::line_item_groups::{self, dsl::*};
use crate::{db::DbPool, error::BudgieError};

use super::Result;
use actix_web::web;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, AsChangeset, Queryable)]
#[diesel(table_name = line_item_groups)]
pub struct NewLineItemGroupPayload {
    budget_id: i32,
    name: String,
}

pub async fn get_line_item_group(
    path: web::Path<i32>,
    db: web::Data<DbPool>,
) -> Result<model::LineItemGroup> {
    let group_id = path.into_inner();
    let mut conn = db.conns.lock().unwrap().get().unwrap();

    let results = line_item_groups
        .filter(id.eq(group_id))
        .limit(1)
        .load::<model::LineItemGroup>(&mut conn)?;

    if let Some(group) = results.get(0) {
        Ok(web::Json(group.clone()))
    } else {
        Err(BudgieError::ResourceNotFound(format!(
            "Line Item Group with id {group_id} not found"
        )))
    }
}

pub async fn new_line_item_group(
    new_group: web::Json<NewLineItemGroupPayload>,
    db: web::Data<DbPool>,
) -> Result<model::LineItemGroup> {
    let new = new_group.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();
    let created = model::LineItemGroup::create(conn, new.budget_id, &new.name)?;
    Ok(web::Json(created))
}

pub async fn update_line_item_group(
    path: web::Path<i32>,
    update_group: web::Json<NewLineItemGroupPayload>,
    db: web::Data<DbPool>,
) -> Result<model::LineItemGroup> {
    let new = update_group.into_inner();
    let group_id = path.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();

    let result = diesel::update(line_item_groups::table)
        .filter(id.eq(group_id))
        .set(new)
        .get_result::<model::LineItemGroup>(conn)?;

    Ok(web::Json(result))
}

pub async fn delete_line_item_group(
    path: web::Path<i32>,
    db: web::Data<DbPool>,
) -> Result<model::LineItemGroup> {
    let group_id = path.into_inner();
    let conn = &mut db.conns.lock().unwrap().get().unwrap();

    let result = diesel::delete(line_item_groups.filter(id.eq(group_id)))
        .get_result::<model::LineItemGroup>(conn)?;
    Ok(web::Json(result))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/line_item_group/{group_id}")
            .route(web::get().to(get_line_item_group))
            .route(web::post().to(update_line_item_group))
            .route(web::delete().to(delete_line_item_group)),
    )
    .service(web::resource("/line_item_group").route(web::post().to(new_line_item_group)));
}
