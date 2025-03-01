use lib::{get_ac_rate, ContestType, ShieldsResponseBody, UserId};
use once_cell::sync::Lazy;
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

static ATCODER_REQUEST_TIME_HISTORY: Lazy<Mutex<VecDeque<Instant>>> = Lazy::new(|| {
    let m = VecDeque::new();
    Mutex::new(m)
});

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(request: Request) -> Result<Response<Body>, Error> {
    // get user_id & contest_type from query-string
    let url = Url::parse(&request.uri().to_string()).map_err(|_| "failed parse uri")?;
    let query_map = url.query_pairs().into_owned().collect::<HashMap<_, _>>();
    let user_id = match query_map
        .get("user_id")
        .and_then(|u| UserId::try_from(u.as_ref()).ok())
    {
        Some(user_id) => user_id,
        _ => return not_found_response("'user_id' param not found or invalid value".into()),
    };
    let contest_type = match query_map.get("contest_type") {
        Some(contest_type) => match ContestType::try_from(contest_type.as_ref()) {
            Ok(contest_type) => contest_type,
            Err(_) => return not_found_response("'contest_type' param is invalid".into()),
        },
        None => ContestType::Algorithm,
    };

    // check rate-limit
    if !check_atcoder_rate_limit() {
        return too_many_requests_response("rate limit has been reached".into());
    }

    let rate = get_ac_rate(&user_id, contest_type).map_err(|_| "failed get AtCoder rate")?;
    let body = ShieldsResponseBody::new(contest_type, rate);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json; charset=utf-8")
        .header("Cache-Control", "max-age=0, s-maxage=86400")
        .body(
            serde_json::to_string(&body)
                .expect("should convert to JSON string")
                .into(),
        )?)
}

fn check_atcoder_rate_limit() -> bool {
    let now = Instant::now();
    let duration = Duration::from_secs(60);
    // 1分以内の履歴のみ残す
    let mut history = ATCODER_REQUEST_TIME_HISTORY.lock().unwrap();
    history.retain(|t| *t >= now - duration);
    // 1分間に10回までのリクエストを許可する
    let ok = history.len() < 10;
    if ok {
        history.push_back(now);
    }
    ok
}

fn not_found_response(mes: String) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", "text/plain")
        .body(mes.into())?)
}

fn too_many_requests_response(mes: String) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::TOO_MANY_REQUESTS)
        .header("Content-Type", "text/plain")
        .body(mes.into())?)
}
