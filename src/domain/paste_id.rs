use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(Debug, Hash, PartialEq, Eq, Deserialize, Clone)]
pub struct PasteId(String);

const FORBIDDEN_CHARS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

impl PasteId {
    pub fn parse(s: String) -> Result<PasteId, String> {
        if s.trim().is_empty() {
            return Err("not a valid paste id - empty string".to_string());
        }

        if s.graphemes(true).count() > 256 {
            return Err(format!("{} is not a valid paste id - too long", s));
        }

        if s.chars().any(|g| FORBIDDEN_CHARS.contains(&g)) {
            return Err(format!("{} is not a valid paste id - invalid char", s));
        }

        Ok(Self(s))
    }

    // TODO: test to ensure parse works
    pub fn random() -> PasteId {
        PasteId(Uuid::new_v4().to_string())
    }
}

impl AsRef<str> for PasteId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::PasteId;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(PasteId::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(PasteId::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(PasteId::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(PasteId::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(PasteId::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Some name".to_string();
        assert_ok!(PasteId::parse(name));
    }
}
