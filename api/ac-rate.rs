use http::StatusCode;
use std::error::Error;
use util::{get_ac_rate, ContestType};
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
    let rate = get_ac_rate("emanon001", ContestType::Algorithm).unwrap();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(format!("{:?}", rate))
        .expect("Internal Server Error");
    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
