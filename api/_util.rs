use reqwest;
use serde::Deserialize;

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

fn contest_history_url(user_id: &str, contest_type: ContestType) -> String {
    let contest_type_param = match contest_type {
        ContestType::Algorithm => "",
        ContestType::Heuristic => "?contestType=heuristic",
    };
    format!(
        "https://atcoder.jp/users/{}/history/json{}",
        user_id, contest_type_param
    )
}
