#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContestType {
    Algorithm,
    Heuristic,
}

impl TryFrom<&str> for ContestType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "algorithm" => Ok(ContestType::Algorithm),
            "heuristic" => Ok(ContestType::Heuristic),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_string() {
        assert_eq!(
            ContestType::try_from("algorithm"),
            Ok(ContestType::Algorithm)
        );
        assert_eq!(
            ContestType::try_from("Algorithm"),
            Ok(ContestType::Algorithm)
        );
        assert_eq!(
            ContestType::try_from("heuristic"),
            Ok(ContestType::Heuristic)
        );
        assert_eq!(
            ContestType::try_from("Heuristic"),
            Ok(ContestType::Heuristic)
        );
        assert_eq!(ContestType::try_from("invalid_type"), Err(()));
    }
}
