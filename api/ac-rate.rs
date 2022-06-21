use http::StatusCode;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use url::Url;
use util::{get_ac_rate, ContestType, ShileldsResponseBody};
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    // get user_id & contest_type from query-string
    let url = match Url::parse(&request.uri().to_string()) {
        Ok(url) => url,
        Err(_) => return Err(VercelError::new("failed parse uri")),
    };
    let query_map = url.query_pairs().into_owned().collect::<HashMap<_, _>>();
    let user_id = match query_map.get("user_id") {
        Some(user_id) if !user_id.trim().is_empty() => user_id.trim(),
        _ => return Ok(not_found_response("'user_id' param not found".into())),
    };
    let contest_type = match query_map.get("contest_type") {
        Some(contest_type) => match contest_type.to_ascii_lowercase().as_ref() {
            "algorighm" => ContestType::Algorithm,
            "heuristic" => ContestType::Heuristic,
            _ => return Ok(not_found_response("'contest_type' param is invalid".into())),
        },
        None => ContestType::Algorithm,
    };

    let rate = match get_ac_rate(user_id, contest_type) {
        Ok(rate) => rate,
        Err(_) => return Err(VercelError::new("failed get atcoder rate".into())),
    };

    let body = ShileldsResponseBody::new_ac_rate_response(contest_type, rate);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(serde_json::to_string(&body).unwrap())
        .expect("Internal Server Error");
    Ok(response)
}

fn not_found_response(mes: String) -> Response<String> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", "text/plain")
        .body(mes)
        .unwrap()
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
