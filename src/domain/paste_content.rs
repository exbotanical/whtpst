use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Deserialize)]
pub struct PasteContent(String);

impl PasteContent {
    pub fn parse(s: String) -> Result<PasteContent, String> {
        if s.trim().is_empty() {
            return Err(format!("{} is not valid paste content - empty string", s));
        }

        if s.graphemes(true).count() > 256 {
            return Err(format!("{} is not valid paste content - too long", s));
        }

        Ok(Self(s))
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
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(PasteContent::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(PasteContent::parse(name));
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
