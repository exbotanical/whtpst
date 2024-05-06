use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder};

use crate::{dao::Repository, domain::PasteId};

#[tracing::instrument(
    name = "Retrieving a paste",
    skip(repo),
    fields(
        raw_paste_id = %raw_paste_id,
    )
)]
pub async fn get_paste<R: Repository>(
    raw_paste_id: web::Path<String>,
    repo: web::Data<Mutex<R>>,
) -> impl Responder {
    let raw_paste_id = raw_paste_id.into_inner();
    let paste_id = match PasteId::parse(raw_paste_id) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

    return match repo
        .into_inner()
        .lock()
        .expect("failed to acquire mutex lock")
        .find_one(paste_id)
    {
        Ok(p) => HttpResponse::Ok().body(p.as_ref().to_owned()),
        Err(e) => HttpResponse::from_error(e),
    };
}
