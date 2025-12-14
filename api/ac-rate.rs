use lib::{ContestType, ShieldsResponseBody, UptrashRateLimiter, UserId, get_ac_rate};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::time::Duration;
use vercel_runtime::{Error, Request, Response, ResponseBody, run, service_fn};

const GLOBAL_RATE_LIMIT_KEY: &str = "ac-rate:global";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service = service_fn(handler);
    run(service).await
}

async fn handler(request: Request) -> Result<Response<ResponseBody>, Error> {
    let rate_limiter = match UptrashRateLimiter::from_env(Duration::from_secs(60), 10) {
        Ok(limiter) => limiter,
        Err(err) => {
            eprintln!("failed to construct Uptrash rate limiter: {err}");
            return internal_server_error_response("rate limiter is not configured".into());
        }
    };

    // get user_id & contest_type from query-string
    let query_map = request
        .uri()
        .query()
        .map(|q| url::form_urlencoded::parse(q.as_bytes()))
        .map(|parsed| parsed.into_owned().collect::<HashMap<_, _>>())
        .unwrap_or_default();
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

    match rate_limiter.check_and_record(GLOBAL_RATE_LIMIT_KEY).await {
        Ok(true) => {}
        Ok(false) => return too_many_requests_response("rate limit has been reached".into()),
        Err(err) => {
            eprintln!("failed to verify Uptrash rate limit: {err}");
            return internal_server_error_response("failed to check rate limit".into());
        }
    }

    let rate = get_ac_rate(&user_id, contest_type)
        .await
        .map_err(|_| "failed get AtCoder rate")?;
    let body = ShieldsResponseBody::new(contest_type, rate);
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json; charset=utf-8")
        .header("Cache-Control", "max-age=0, s-maxage=86400")
        .body(
            serde_json::to_string(&body)
                .expect("should convert to JSON string")
                .into(),
        )?)
}

fn not_found_response(mes: String) -> Result<Response<ResponseBody>, Error> {
    Ok(Response::builder()
        .status(404)
        .header("Content-Type", "text/plain")
        .body(mes.into())?)
}

fn too_many_requests_response(mes: String) -> Result<Response<ResponseBody>, Error> {
    Ok(Response::builder()
        .status(429)
        .header("Content-Type", "text/plain")
        .body(mes.into())?)
}

fn internal_server_error_response(mes: String) -> Result<Response<ResponseBody>, Error> {
    Ok(Response::builder()
        .status(500)
        .header("Content-Type", "text/plain")
        .body(mes.into())?)
}
