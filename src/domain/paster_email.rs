use validator::validate_email;

#[derive(Debug)]
pub struct PasterEmail(String);

impl PasterEmail {
    pub fn parse(s: String) -> Result<PasterEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid email address", s))
        }
    }
}

impl AsRef<str> for PasterEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::PasterEmail;
    use claim::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(PasterEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "somedomain.com".to_string();
        assert_err!(PasterEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(PasterEmail::parse(email));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        PasterEmail::parse(valid_email.0).is_ok()
    }
}
