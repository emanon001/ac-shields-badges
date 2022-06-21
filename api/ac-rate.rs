use http::StatusCode;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use url::Url;
use util::{get_ac_rate, ContestType, ShileldsResponseBody};
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    let url = match Url::parse(&request.uri().to_string()) {
        Ok(url) => url,
        Err(_) => return Ok(not_found_response()),
    };
    let query_map = url.query_pairs().into_owned().collect::<HashMap<_, _>>();
    let user_id = match query_map.get("user_id") {
        Some(user_id) => user_id,
        None => return Ok(not_found_response()),
    };
    let contest_type = match query_map.get("contest_type") {
        Some(contest_type) => match contest_type.to_ascii_lowercase().as_ref() {
            "algorighm" => ContestType::Algorithm,
            "heuristic" => ContestType::Heuristic,
            _ => return Ok(not_found_response()),
        },
        None => ContestType::Algorithm,
    };
    // TODO: check error
    let rate = get_ac_rate(user_id, contest_type).unwrap();
    let body = ShileldsResponseBody::new_ac_rate_response(contest_type, rate);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(serde_json::to_string(&body).unwrap())
        .expect("Internal Server Error");
    Ok(response)
}

fn not_found_response() -> Response<String> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", "text/plain")
        .body("Not Found".to_string())
        .unwrap()
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
