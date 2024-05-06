use core::fmt;

use actix_web::{HttpResponse, ResponseError};

use crate::domain::{NewPaste, PasteContent, PasteId};

#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
    WriteFailure(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::WriteFailure(msg) => write!(f, "Failed to write: {}", msg),
        }
    }
}

impl ResponseError for RepositoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RepositoryError::NotFound(_) => HttpResponse::NotFound().body(self.to_string()),
            RepositoryError::WriteFailure(_) => HttpResponse::BadRequest().body(self.to_string()),
        }
    }
}

pub trait Repository: Sync + Send + 'static {
    fn new() -> Self;
    fn find_one(&self, id: PasteId) -> Result<PasteContent, RepositoryError>;
    fn insert(&mut self, entity: NewPaste) -> Result<(), RepositoryError>;
}

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
