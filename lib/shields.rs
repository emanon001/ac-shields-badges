use serde::Serialize;

use crate::contest_type::ContestType;
use crate::rate::Rate;

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShieldsResponseBody {
    schema_version: u32,
    label: String,
    message: String,
    color: String,
}

impl ShieldsResponseBody {
    pub fn new_ac_rate_response(contest_type: ContestType, rate: Option<Rate>) -> Self {
        let label = format!(
            "AtCoder{}",
            match contest_type {
                ContestType::Algorithm => "Ⓐ",
                ContestType::Heuristic => "Ⓗ",
            }
        );
        let (message, color) = match rate {
            Some(rate) => {
                let message = rate.to_string();
                let color = match rate.0 {
                    ..=399 => "808080",
                    400..=799 => "804000",
                    800..=1199 => "008000",
                    1200..=1599 => "00C0C0",
                    1600..=1999 => "0000FF",
                    2000..=2399 => "C0C000",
                    2400..=2799 => "FF8000",
                    _ => "FF0000",
                }
                .to_string();
                (message, color)
            }
            None => ("-".to_owned(), "000000".to_owned()),
        };

        Self {
            schema_version: 1,
            label,
            message,
            color,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0, "808080")]
    #[case(399, "808080")]
    #[case(400, "804000")]
    #[case(799, "804000")]
    #[case(800, "008000")]
    #[case(1199, "008000")]
    #[case(1200, "00C0C0")]
    #[case(1599, "00C0C0")]
    #[case(1600, "0000FF")]
    #[case(1999, "0000FF")]
    #[case(2000, "C0C000")]
    #[case(2399, "C0C000")]
    #[case(2400, "FF8000")]
    #[case(2799, "FF8000")]
    #[case(2800, "FF0000")]
    #[case(4200, "FF0000")]
    fn test_new_ac_rate_response_algorithm(#[case] rate: u32, #[case] color: &str) {
        assert_eq!(
            ShieldsResponseBody::new_ac_rate_response(ContestType::Algorithm, Some(Rate(rate))),
            ShieldsResponseBody {
                schema_version: 1,
                label: "AtCoderⒶ".to_string(),
                message: rate.to_string(),
                color: color.to_string(),
            }
        );
    }

    #[test]
    fn test_new_ac_rate_response_algorithm_user_has_not_competed_in_a_rated_yet() {
        assert_eq!(
            ShieldsResponseBody::new_ac_rate_response(ContestType::Algorithm, None),
            ShieldsResponseBody {
                schema_version: 1,
                label: "AtCoderⒶ".to_string(),
                message: "-".to_owned(),
                color: "000000".to_owned(),
            }
        );
    }

    #[rstest]
    #[case(0, "808080")]
    #[case(399, "808080")]
    #[case(400, "804000")]
    #[case(799, "804000")]
    #[case(800, "008000")]
    #[case(1199, "008000")]
    #[case(1200, "00C0C0")]
    #[case(1599, "00C0C0")]
    #[case(1600, "0000FF")]
    #[case(1999, "0000FF")]
    #[case(2000, "C0C000")]
    #[case(2399, "C0C000")]
    #[case(2400, "FF8000")]
    #[case(2799, "FF8000")]
    #[case(2800, "FF0000")]
    #[case(4200, "FF0000")]
    fn test_new_ac_rate_response_heuristic(#[case] rate: u32, #[case] color: &str) {
        assert_eq!(
            ShieldsResponseBody::new_ac_rate_response(ContestType::Heuristic, Some(Rate(rate))),
            ShieldsResponseBody {
                schema_version: 1,
                label: "AtCoderⒽ".to_string(),
                message: rate.to_string(),
                color: color.to_string(),
            }
        );
    }

    #[test]
    fn test_new_ac_rate_response_heuristic_user_has_not_competed_in_a_rated_yet() {
        assert_eq!(
            ShieldsResponseBody::new_ac_rate_response(ContestType::Heuristic, None),
            ShieldsResponseBody {
                schema_version: 1,
                label: "AtCoderⒽ".to_string(),
                message: "-".to_owned(),
                color: "000000".to_owned(),
            }
        );
    }
}
