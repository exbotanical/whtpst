use std::sync::Mutex;

use actix_web::{
    web::{self, Bytes},
    HttpResponse, Responder,
};

use crate::{
    dao::Repository,
    domain::{NewPaste, PasteContent, PasteId},
};
// use sqlx::PgPool;

// #[derive(serde::Deserialize)]
// pub struct PasteFormData {
//     email: String,
//     name: String,
// }

// impl TryFrom<PasteFormData> for NewPaste {
//     type Error = String;

//     fn try_from(value: PasteFormData) -> Result<Self, Self::Error> {
//         let name = PasteId::parse(value.name)?;
//         let email = PasteContent::parse(value.email)?;

//         Ok(NewPaste { name, email })
//     }
// }

// #[tracing::instrument(
//     name = "Adding a new subscriber",
//     skip(form, pool),
//     fields(
//         subscriber_name = %form.name,
//         subscriber_email = %form.email
//     )
// )]
// pub async fn subscribe(form: web::Json<PasteFormData>, pool: web::Data<PgPool>) -> impl Responder {
//     let new_subscriber = match form.0.try_into() {
//         Ok(subscriber) => subscriber,
//         Err(_) => return HttpResponse::BadRequest(),
//     };

//     match insert_subscriber(&pool, &new_subscriber).await {
//         Ok(_) => HttpResponse::Ok(),
//         Err(_) => HttpResponse::InternalServerError(),
//     }
// }

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

    let paste_content = match String::from_utf8(bytes.to_vec()) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().body(""),
    };

    let paste_content = match PasteContent::parse(paste_content) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

    let new_paste = NewPaste {
        id: paste_id.clone(),
        content: paste_content,
    };

    match repo.into_inner().lock().expect("msg").insert(new_paste) {
        Ok(_) => (),
        Err(e) => {
            return HttpResponse::BadRequest().body(format!("Error inserting paste: {:#?}", e))
        }
    }

    HttpResponse::Ok().body(paste_id.as_ref().to_owned())
}

#[tracing::instrument(
    name = "Adding a new paste with a generated id",
    skip(repo),
    fields(
        // raw_paste_id = %raw_paste_id,
    )
)]
pub async fn create_paste_random_id<R: Repository>(
    bytes: Bytes,
    repo: web::Data<Mutex<R>>,
) -> impl Responder {
    let paste_id = PasteId::random();

    // TODO: extract
    let paste_content = match String::from_utf8(bytes.to_vec()) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().body(""),
    };

    let paste_content = match PasteContent::parse(paste_content) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

    let new_paste = NewPaste {
        id: paste_id.clone(),
        content: paste_content,
    };

    match repo.into_inner().lock().expect("msg").insert(new_paste) {
        Ok(_) => (),
        Err(e) => {
            return HttpResponse::BadRequest().body(format!("Error inserting paste: {:#?}", e))
        }
    }

    HttpResponse::Ok().body(paste_id.as_ref().to_owned())
}

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

    let paste_content = match repo.into_inner().lock().expect("TODO:").find_one(paste_id) {
        Ok(p) => p,
        Err(e) => {
            // TODO: impl return type of RepositoryError
            return HttpResponse::BadRequest().body(format!("Error fetching paste: {:#?}", e));
        }
    };

    HttpResponse::Ok().body(paste_content.as_ref().to_owned())
}

// #[tracing::instrument(
//     name = "Saving new subscriber details into database",
//     skip(pool, new_subscriber)
// )]
// pub async fn insert_subscriber(
//     pool: &PgPool,
//     new_subscriber: &NewPaste,
// ) -> Result<(), sqlx::Error> {
//     sqlx::query!(
//         r#"
//     INSERT INTO subscriptions (id, email, name, subscribed_at)
//     VALUES ($1, $2, $3, $4)
//     "#,
//         Uuid::new_v4(),
//         new_subscriber.email.as_ref(),
//         new_subscriber.name.as_ref(),
//         Utc::now()
//     )
//     .execute(pool)
//     .await
//     .map_err(|e| {
//         tracing::error!("Failed to execute query {:?}", e);
//         e
//     })?;
//     Ok(())
// }
