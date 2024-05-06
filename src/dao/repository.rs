use crate::domain::{NewPaste, PasteContent, PasteId};

#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
}

pub trait Repository: Sync + Send + 'static {
    fn new() -> Self;
    fn find_one(&self, id: PasteId) -> Result<PasteContent, RepositoryError>;
    fn insert(&mut self, entity: NewPaste) -> Result<(), RepositoryError>;
}
