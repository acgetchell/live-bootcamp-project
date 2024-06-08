use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email, String> {
        if s.validate_email() {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid email", s))
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
        assert_eq!(email, Err(" is not a valid email".to_string()));
    }

    #[test]
    fn mail_missing_at_symbol_is_rejected() {
        let email = Email::parse("test.com".to_string());
        assert_eq!(email, Err("test.com is not a valid email".to_string()));
    }

    #[test]
    fn mail_missing_domain_is_rejected() {
        let email = Email::parse("test@".to_string());
        assert_eq!(email, Err("test@ is not a valid email".to_string()));
    }

    #[test]
    fn mail_missing_address_is_rejected() {
        let email = Email::parse("@test.com".to_string());
        assert_eq!(email, Err("@test.com is not a valid email".to_string()));
    }

    #[test]
    fn mail_with_at_symbol_and_domain_is_accepted() {
        let email = Email::parse("test@test.com".to_string());
        assert!(email.is_ok());
    }
}
