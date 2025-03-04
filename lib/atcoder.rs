use easy_scraper::Pattern;
use reqwest;
use url::Url;

use crate::contest_type::ContestType;
use crate::rate::Rate;
use crate::user_id::UserId;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    RequestError(#[source] reqwest::Error),
    #[error("Failed to parse rate: {0}")]
    ParseError(#[source] std::num::ParseIntError),
    #[error("Rate not found in the response")]
    RateNotFound,
    #[error("Pattern matching failed")]
    PatternError,
}

pub async fn get_ac_rate(
    user_id: &UserId,
    contest_type: ContestType,
) -> Result<Option<Rate>, Error> {
    let client = reqwest::Client::new();
    let doc = client
        .get(user_profile_url(user_id, contest_type))
        .send()
        .await
        .map_err(Error::RequestError)?
        .text()
        .await
        .map_err(Error::RequestError)?;
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
    )
    .expect("Pattern should be valid");
    let ms = pat.matches(&doc);
    match ms.first() {
        Some(m) => {
            let rate: u32 = m["rate"].parse().map_err(Error::ParseError)?;
            Ok(Some(Rate::new(rate)))
        }
        None => Err(Error::RateNotFound),
    }
}

fn user_profile_url(user_id: &UserId, contest_type: ContestType) -> Url {
    let mut params: Vec<(&str, &str)> = Vec::new();
    params.push(("lang", "en"));
    params.push((
        "contestType",
        match contest_type {
            ContestType::Heuristic => "heuristic",
            ContestType::Algorithm => "algo", // "algorithm" ではなく "algo" を指定する必要がある
        },
    ));
    Url::parse_with_params(&format!("https://atcoder.jp/users/{}", user_id), &params).unwrap()
}
