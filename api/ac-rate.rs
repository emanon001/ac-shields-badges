use http::StatusCode;
use once_cell::sync::Lazy;
use serde_json;
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;
use std::error::Error;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use url::Url;
use util::{get_ac_rate, ContestType, ShieldsResponseBody, UserId};
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

static ATCODER_REQUEST_TIME_HISTORY: Lazy<Mutex<VecDeque<Instant>>> = Lazy::new(|| {
    let m = VecDeque::new();
    Mutex::new(m)
});

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    // get user_id & contest_type from query-string
    let url = match Url::parse(&request.uri().to_string()) {
        Ok(url) => url,
        Err(_) => return Err(VercelError::new("failed parse uri")),
    };
    let query_map = url.query_pairs().into_owned().collect::<HashMap<_, _>>();
    let user_id = match query_map
        .get("user_id")
        .and_then(|u| UserId::try_from(u.as_ref()).ok())
    {
        Some(user_id) => user_id,
        _ => {
            return Ok(not_found_response(
                "'user_id' param not found or invalid value".into(),
            ))
        }
    };
    let contest_type = match query_map.get("contest_type") {
        Some(contest_type) => match ContestType::try_from(contest_type.as_ref()) {
            Ok(contest_type) => contest_type,
            Err(_) => return Ok(not_found_response("'contest_type' param is invalid".into())),
        },
        None => ContestType::Algorithm,
    };

    // check rate-limit
    if !check_atcoder_rate_limit() {
        return Ok(too_many_requests_response(
            "rate limit has been reached".into(),
        ));
    }

    let rate = match get_ac_rate(&user_id, contest_type) {
        Ok(rate) => rate,
        Err(_) => return Err(VercelError::new("failed get atcoder rate".into())),
    };

    let body = ShieldsResponseBody::new_ac_rate_response(contest_type, rate);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json; charset=utf-8")
        .header("Cache-Control", "max-age=0, s-maxage=86400")
        .body(serde_json::to_string(&body).unwrap())
        .expect("Internal Server Error");
    Ok(response)
}

fn check_atcoder_rate_limit() -> bool {
    let now = Instant::now();
    let duration = Duration::from_secs(60);
    // 1分以内の履歴のみ残す
    let mut history = ATCODER_REQUEST_TIME_HISTORY.lock().unwrap();
    loop {
        match history.pop_front() {
            Some(t) => {
                if t >= now - duration {
                    // restore
                    history.push_front(t);
                    break;
                }
            }
            _ => break,
        }
    }
    // 1分間に10回までのリクエストを許可する
    let ok = history.len() < 10;
    if ok {
        history.push_back(now);
    }
    ok
}

fn not_found_response(mes: String) -> Response<String> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", "text/plain")
        .body(mes)
        .unwrap()
}

fn too_many_requests_response(mes: String) -> Response<String> {
    Response::builder()
        .status(StatusCode::TOO_MANY_REQUESTS)
        .header("Content-Type", "text/plain")
        .body(mes)
        .unwrap()
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
