use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub struct UserId(String);

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

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abc")] // 最小長
    #[case("Abc")] // 大文字を含む
    #[case("123")] // 数字のみ
    #[case("abc123")] // 英数字混在
    #[case("user_name")] // アンダースコアを含む
    #[case("0123456789123456")] // 最大長
    fn test_valid_user_id(#[case] input: &str) {
        let user_id =
            UserId::try_from(input).expect(&format!("Should accept valid user ID: '{}'", input));
        assert_eq!(
            user_id.to_string(),
            input,
            "UserId created from '{}' should display as same string",
            input
        );
    }

    #[rstest]
    #[case("ab")] // 短すぎる（2文字）
    #[case("01234567891234567")] // 長すぎる（17文字）
    #[case("")] // 空文字列
    #[case(" ")] // スペースのみ
    #[case("user-name")] // 無効な文字（ハイフン）
    #[case("user.name")] // 無効な文字（ドット）
    #[case("user@name")] // 無効な文字（@）
    #[case("あいう")] // 非ASCII文字
    fn test_invalid_user_id(#[case] input: &str) {
        assert_eq!(
            UserId::try_from(input),
            Err(()),
            "Expected Err for invalid input: '{}'",
            input
        );
    }

    #[test]
    fn test_trim_whitespace() {
        let input = "  user123  ";
        let user_id = UserId::try_from(input)
            .expect("Should create UserId from input with surrounding whitespace");
        assert_eq!(
            user_id.to_string(),
            "user123",
            "Whitespace should be trimmed"
        );
    }
}
