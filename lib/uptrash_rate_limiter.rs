use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const RATE_LIMIT_SCRIPT: &str = r#"
local current = redis.call('INCR', KEYS[1])
if current == 1 then
  redis.call('EXPIRE', KEYS[1], tonumber(ARGV[1]))
end
return current
"#;

#[derive(Clone)]
pub struct UptrashRateLimiter {
    client: Client,
    endpoint: String,
    token: String,
    window: Duration,
    request_limit_per_window: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("environment variable '{0}' is not set")]
    MissingEnvVar(&'static str),
    #[error("failed to build HTTP client: {0}")]
    ClientBuild(#[source] reqwest::Error),
    #[error("failed to call Uptrash Redis: {0}")]
    Request(#[source] reqwest::Error),
    #[error("failed to parse Uptrash response: {0}")]
    Deserialize(#[source] serde_json::Error),
    #[error("error response from Uptrash: {0}, {1}")]
    ErrorResponse(reqwest::StatusCode, String),
    #[error("error pipeline API response: {0}")]
    ErrorPipelineApiResponse(String),
    #[error("invalid Uptrash response format")]
    InvalidResponseFormat,
    #[error("window duration must be greater than 0 seconds")]
    InvalidWindowDuration,
}

impl UptrashRateLimiter {
    pub fn from_env(window: Duration, request_limit_per_window: u64) -> Result<Self, Error> {
        if window.is_zero() {
            return Err(Error::InvalidWindowDuration);
        }
        let endpoint =
            env::var("KV_REST_API_URL").map_err(|_| Error::MissingEnvVar("KV_REST_API_URL"))?;
        let token =
            env::var("KV_REST_API_TOKEN").map_err(|_| Error::MissingEnvVar("KV_REST_API_TOKEN"))?;
        let client = Client::builder().build().map_err(Error::ClientBuild)?;
        Ok(Self {
            client,
            endpoint,
            token,
            window,
            request_limit_per_window,
        })
    }

    pub async fn check_and_record(&self, key: &str) -> Result<bool, Error> {
        let window_seconds = self.window.as_secs().max(1);
        let bucket = current_minute_bucket(self.window);
        let redis_key = format!("{key}:{bucket}");
        let payload = serde_json::json!([[
            "EVAL",
            RATE_LIMIT_SCRIPT,
            "1",
            redis_key,
            window_seconds.to_string(),
        ]]);
        let url = format!("{}/pipeline", self.endpoint.trim_end_matches('/'));
        let resp = self
            .client
            .post(url)
            .bearer_auth(&self.token)
            .json(&payload)
            .send()
            .await
            .map_err(Error::Request)?;

        let status = resp.status();
        let text = resp.text().await.map_err(Error::Request)?;
        if !status.is_success() {
            return Err(Error::ErrorResponse(status, text));
        }

        let body: Vec<PipelineItemResponse<u64>> =
            serde_json::from_str(&text).map_err(Error::Deserialize)?;
        let parsed = body.first().ok_or(Error::InvalidResponseFormat)?;
        match (parsed.result, &parsed.error) {
            (Some(count), None) => Ok(count <= self.request_limit_per_window),
            (_, Some(err_msg)) => Err(Error::ErrorPipelineApiResponse(err_msg.to_owned())),
            _ => Err(Error::InvalidResponseFormat),
        }
    }
}

#[derive(Deserialize)]
struct PipelineItemResponse<T> {
    #[serde(default)]
    result: Option<T>,
    #[serde(default)]
    error: Option<String>,
}

fn current_minute_bucket(window: Duration) -> u64 {
    let seconds = window.as_secs().max(1);
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before UNIX_EPOCH")
        .as_secs()
        / seconds
}
