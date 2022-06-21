use reqwest;
use serde::Deserialize;
use url::Url;

#[derive(Clone, Copy, Debug)]
pub struct Rate(u32);

#[derive(Copy, Clone, Debug)]
pub enum ContestType {
    Algorithm,
    Heuristic,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ContestHistoryResponse {
    new_rating: u32,
}

pub fn get_ac_rate(
    user_id: &str,
    contest_type: ContestType,
) -> Result<Option<Rate>, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(contest_history_url(user_id, contest_type))?
        .json::<Vec<ContestHistoryResponse>>()
        .unwrap();
    if let Some(latest) = resp.last() {
        Ok(Some(Rate(latest.new_rating)))
    } else {
        Ok(None)
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
