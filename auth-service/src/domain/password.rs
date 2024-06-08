#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Password(String);

impl Password {
    pub fn parse(s: String) -> Result<Password, String> {
        if s.len() < 8 {
            Err("Password must be at least 8 characters long".to_string())
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    #[test]
    fn password_shorter_than_8_characters_is_rejected() {
        let password = Password::parse("1234567".to_string());
        assert_eq!(
            password,
            Err("Password must be at least 8 characters long".to_string())
        );
    }

    #[test]
    fn password_8_characters_long_is_accepted() {
        let password = Password::parse("12345678".to_string());
        assert!(password.is_ok());
    }
}
