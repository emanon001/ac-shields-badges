use easy_scraper::Pattern;
use regex::Regex;
use reqwest;
use serde::Serialize;
use std::convert::TryFrom;
use url::Url;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rate(u32);

impl std::fmt::Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContestType {
    Algorithm,
    Heuristic,
}

impl TryFrom<&str> for ContestType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_ref() {
            "algorithm" => Ok(Self::Algorithm),
            "heuristic" => Ok(Self::Heuristic),
            _ => Err(()),
        }
    }
}

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

pub fn get_ac_rate(
    user_id: &UserId,
    contest_type: ContestType,
) -> Result<Option<Rate>, Box<dyn std::error::Error>> {
    let doc = reqwest::blocking::get(user_profile_url(user_id, contest_type))?.text()?;
    if doc.contains("This user has not competed in a rated contest yet.") {
        return Ok(None);
    }

    let pat = Pattern::new(
        r#"
        <table>
            <tbody>
                <tr>
                    <th>Rating</th>
                    <td>
                        <img>
                        <span>{{rate}}</span>
                    </td>
                </tr>
            </tbody>
        </table>"#,
    )?;
    let ms = pat.matches(&doc);
    match ms.first() {
        Some(m) => {
            let rate: u32 = m["rate"].parse()?;
            Ok(Some(Rate(rate)))
        }
        None => Err("rate not found".into()),
    }
}

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

fn user_profile_url(user_id: &UserId, contest_type: ContestType) -> Url {
    let mut params: Vec<(&str, &str)> = Vec::new();
    params.push(("lang", "en"));
    match contest_type {
        ContestType::Heuristic => {
            params.push(("contestType", "heuristic"));
        }
        _ => {}
    };
    Url::parse_with_params(&format!("https://atcoder.jp/users/{}", user_id.0), &params).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    mod contest_type {
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

    mod user_id {
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

    mod shields_response_body {
        use super::*;

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
