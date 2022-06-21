use http::StatusCode;
use serde_json;
use std::error::Error;
use util::{get_ac_rate, ContestType, ShileldsResponseBody};
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
    // TODO: check result
    // TODO: get user_id & contest_type from request params
    let user_id = "emanon001";
    let contest_type = ContestType::Algorithm;
    let rate = get_ac_rate(user_id, contest_type).unwrap();
    let body = ShileldsResponseBody::new_ac_rate_response(contest_type, rate);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(serde_json::to_string(&body).unwrap())
        .expect("Internal Server Error");
    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
