use std::sync::Mutex;

use actix_web::{
    web::{self, Bytes},
    HttpResponse, Responder,
};

use crate::{
    dao::Repository,
    domain::{NewPaste, PasteContent, PasteId},
};

#[tracing::instrument(name = "Adding a new paste with a generated id", skip(repo))]
pub async fn create_paste_sans_id<R: Repository>(
    bytes: Bytes,
    repo: web::Data<Mutex<R>>,
) -> impl Responder {
    let paste_id = PasteId::random();

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
