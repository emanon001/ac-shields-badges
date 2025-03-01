use easy_scraper::Pattern;
use reqwest;
use url::Url;

use crate::contest_type::ContestType;
use crate::rate::Rate;
use crate::user_id::UserId;

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
