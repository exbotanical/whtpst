use std::collections::HashMap;

use super::{repository::RepositoryError, Repository};
use crate::domain::{PasteContent, PasteId};

pub struct InMemoryRepository {
    data: HashMap<PasteId, PasteContent>,
}

impl Repository for InMemoryRepository {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn find_one(&self, id: PasteId) -> Result<PasteContent, super::repository::RepositoryError> {
        match self.data.get(&id) {
            Some(d) => Ok(d.to_owned()),
            None => Err(RepositoryError::NotFound(id.as_ref().to_owned())),
        }
    }

    fn insert(&mut self, entity: crate::domain::NewPaste) -> Result<(), RepositoryError> {
        self.data.insert(entity.id, entity.content);
        Ok(())
    }
}
