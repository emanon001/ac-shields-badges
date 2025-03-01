use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub struct UserId(pub(crate) String);

impl TryFrom<&str> for UserId {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        let re = Regex::new(r"\A[_a-zA-Z0-9]{3,16}\z").unwrap();
        if re.is_match(value) {
            Ok(UserId(value.into()))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_string() {
        assert_eq!(UserId::try_from("abc"), Ok(UserId("abc".into())));
        assert_eq!(UserId::try_from("Abc"), Ok(UserId("Abc".into())));
        assert_eq!(UserId::try_from("123"), Ok(UserId("123".into())));
        assert_eq!(UserId::try_from("abc123"), Ok(UserId("abc123".into())));
        assert_eq!(
            UserId::try_from("0123456789123456"),
            Ok(UserId("0123456789123456".into()))
        );
        assert_eq!(UserId::try_from("ab"), Err(()));
        assert_eq!(UserId::try_from("01234567891234567"), Err(()));
    }
}
