mod schema;

use anyhow::bail;
use anyhow::{Context, Result};
use reqwest::{self, Url};
use schema::{request, response};
use serde_json::Value;

const BASE_URL: &str = "https://app.divera247.com/api/";
const ENDPOINT_LOGIN: &str = "v2/auth/login";

pub fn login(username: String, password: String) -> Result<String> {
    let url = create_url(BASE_URL, ENDPOINT_LOGIN);
    let body = request::LoginRequest {
        login: request::Login { username, password },
    };
    let response = reqwest::blocking::Client::new()
        .post(url)
        .json(&body)
        .send()
        .with_context(|| format!("Failed to send login request to divera247"))?;

    let response_text = response.text().unwrap();
    let raw_response: Value = serde_json::from_str(&response_text).unwrap();

    if !raw_response.get("success").unwrap().as_bool().unwrap() {
        bail!("Failed to login {}", raw_response.get("errors").unwrap());
    }

    let response: response::SuccessResponse<response::Login> =
        serde_json::from_str(&response_text)?;

    Ok(response.data.user.access_token)
}

fn create_url(base: &str, endpoint: &str) -> Url {
    Url::parse(base).unwrap().join(endpoint).unwrap()
}
