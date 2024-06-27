use color_eyre::eyre::{eyre, Result};
use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email> {
        if s.validate_email() {
            Ok(Self(s))
        } else {
            Err(eyre!(format!("{} is not a valid email", s)))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;

    #[test]
    fn empty_string_is_rejected() {
        let email = Email::parse("".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn mail_missing_at_symbol_is_rejected() {
        let email = Email::parse("test.com".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn mail_missing_domain_is_rejected() {
        let email = Email::parse("test@".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn mail_missing_address_is_rejected() {
        let email = Email::parse("@test.com".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn mail_with_at_symbol_and_domain_is_accepted() {
        let email = Email::parse("test@test.com".to_string());
        assert!(email.is_ok());
    }
}
