use std::sync::Mutex;

use actix_web::{
    web::{self, Bytes},
    HttpResponse, Responder,
};

use crate::{
    dao::Repository,
    domain::{NewPaste, PasteContent, PasteId},
};

#[tracing::instrument(
    name = "Adding a new paste",
    skip(repo),
    fields(
        raw_paste_id = %raw_paste_id,
    )
)]
pub async fn create_paste<R: Repository>(
    raw_paste_id: web::Path<String>,
    bytes: Bytes,
    repo: web::Data<Mutex<R>>,
) -> impl Responder {
    let raw_paste_id = raw_paste_id.into_inner();
    let paste_id = match PasteId::parse(raw_paste_id) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

    let paste_content = match PasteContent::parse_bytes(bytes) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

    let new_paste = NewPaste {
        id: paste_id.clone(),
        content: paste_content,
    };

    return match repo
        .into_inner()
        .lock()
        .expect("failed to acquire mutex lock")
        .insert(new_paste)
    {
        Ok(_) => HttpResponse::Ok().body(paste_id.as_ref().to_owned()),
        Err(e) => HttpResponse::from_error(e),
    };
}
