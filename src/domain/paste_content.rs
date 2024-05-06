use actix_web::web::Bytes;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PasteContent(String);

impl PasteContent {
    pub fn parse(s: String) -> Result<PasteContent, String> {
        if s.trim().is_empty() {
            return Err("not valid paste content - empty string".to_string());
        }

        Ok(Self(s))
    }

    pub fn parse_bytes(bytes: Bytes) -> Result<PasteContent, String> {
        let paste_content = match String::from_utf8(bytes.to_vec()) {
            Ok(p) => p,
            Err(e) => return Err(e.to_owned().to_string()),
        };

        return PasteContent::parse(paste_content);
    }
}

impl AsRef<str> for PasteContent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::PasteContent;
    use claim::{assert_err, assert_ok};

    #[test]
    fn normal_string_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(PasteContent::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(PasteContent::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(PasteContent::parse(name));
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Some name".to_string();
        assert_ok!(PasteContent::parse(name));
    }
}
