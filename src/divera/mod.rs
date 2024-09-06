mod schema;

use anyhow::bail;
use anyhow::Result;
use reqwest::{self, blocking::Response, Url};
use schema::{
    request,
    response::{self, SuccessResponse},
};
use serde::de::DeserializeOwned;
use serde_json::Value;

const BASE_URL: &str = "https://app.divera247.com/api/";
const ENDPOINT_LOGIN: &str = "v2/auth/login";
const ENDPOINT_REPORTTYPES: &str = "v2/reporttypes";

pub fn login(username: String, password: String) -> Result<String> {
    let url = create_url(BASE_URL, ENDPOINT_LOGIN);
    let body = request::LoginRequest {
        login: request::Login { username, password },
    };
    let response = reqwest::blocking::Client::new()
        .post(url)
        .json(&body)
        .send()?;

    let response = parse_response::<response::Login>(response)?;
    Ok(response.data.user.access_token)
}

pub fn report_types(access_token: String) -> Result<()> {
    let url = create_url(BASE_URL, ENDPOINT_REPORTTYPES);
    let response = reqwest::blocking::Client::new()
        .get(url)
        .query(&[("accesskey", access_token)])
        .send()?;

    let report_types = parse_response::<response::ReportTypes>(response)?;
    dbg!(report_types);

    Ok(())
}

fn create_url(base: &str, endpoint: &str) -> Url {
    Url::parse(base).unwrap().join(endpoint).unwrap()
}

fn parse_response<T: DeserializeOwned>(response: Response) -> Result<SuccessResponse<T>> {
    let response_text = response.text()?;
    let raw_response: Value = serde_json::from_str(&response_text).unwrap();

    if !raw_response.get("success").unwrap().as_bool().unwrap() {
        bail!("Request faield {}", raw_response.get("errors").unwrap());
    }

    Ok(serde_json::from_str(&response_text)?)
}
