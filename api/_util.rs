use reqwest;
use serde::{Deserialize, Serialize};
use url::Url;

type Rate = u32;

#[derive(Copy, Clone, Debug)]
pub enum ContestType {
    Algorithm,
    Heuristic,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ContestHistoryResponse {
    is_rated: bool,
    new_rating: u32,
}

pub fn get_ac_rate(
    user_id: &str,
    contest_type: ContestType,
) -> Result<Rate, Box<dyn std::error::Error>> {
    let history_list = reqwest::blocking::get(contest_history_url(user_id, contest_type))?
        .json::<Vec<ContestHistoryResponse>>()?;
    for h in history_list.into_iter().rev() {
        if h.is_rated {
            return Ok(h.new_rating);
        }
    }
    Ok(0)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShileldsResponseBody {
    schema_version: u32,
    label: String,
    message: String,
    color: String,
}

impl ShileldsResponseBody {
    pub fn new_ac_rate_response(contest_type: ContestType, rate: Rate) -> Self {
        let label = format!(
            "AtCoder{}",
            match contest_type {
                ContestType::Algorithm => "Ⓐ",
                ContestType::Heuristic => "Ⓗ",
            }
        );
        let message = rate.to_string();
        let color = match rate {
            0 => "000000",
            1..=399 => "808080",
            400..=799 => "804000",
            800..=1199 => "008000",
            1200..=1599 => "00C0C0",
            1600..=1999 => "0000FF",
            2000..=2399 => "C0C000",
            2400..=2799 => "FF8000",
            _ => "FF0000",
        }
        .to_string();

        Self {
            schema_version: 1,
            label,
            message,
            color,
        }
    }
}

fn contest_history_url(user_id: &str, contest_type: ContestType) -> Url {
    let mut params = Vec::new();
    match contest_type {
        ContestType::Heuristic => {
            params.push(("contestType", "heuristic"));
        }
        _ => {}
    };
    Url::parse_with_params(
        &format!("https://atcoder.jp/users/{}/history/json", user_id),
        &params,
    )
    .unwrap()
}
