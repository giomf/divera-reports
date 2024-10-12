#![allow(unused)]

pub mod schema;

use std::any::type_name;

use anyhow::{bail, Context, Result};
use reqwest::{
    self,
    blocking::{RequestBuilder, Response},
    header::COOKIE,
    Url,
};
use schema::{
    request,
    response::{self, SuccessResponse},
};
use serde::de::DeserializeOwned;
use serde_json::Value;

const BASE_URL: &str = "https://app.divera247.com/api/";
const ENDPOINT_PULL_ALL: &str = "v2/pull/all";
const ENDPOINT_LOGIN: &str = "v2/auth/login";
const ENDPOINT_JWT: &str = "v2/auth/jwt";
const ENDPOINT_REPORTTYPES: &str = "v2/reporttypes";
const ENDPOINT_REPORTS: &str = "v2/reporttypes/reports";
const ENDPOINT_EXPORT_USERS: &str = "v2/management/export-users";
const ENDPOINT_USERS: &str = "users";

pub fn login(username: String, password: String) -> Result<response::Login> {
    let url = create_url(BASE_URL, ENDPOINT_LOGIN);
    let body = request::LoginRequest {
        login: request::Login {
            username,
            password,
            jwt: false,
        },
    };
    let request = reqwest::blocking::Client::new().post(url).json(&body);
    let response = send(request)?;

    let login = handle_response(response).with_context(|| "Failed to handle login response")?;
    Ok(login)
}

pub fn _jwt(access_token: &str) -> Result<response::Jwt> {
    let url = create_url(BASE_URL, ENDPOINT_JWT);
    let request = reqwest::blocking::Client::new()
        .get(url)
        .query(&[("accesskey", access_token)]);
    let response = send(request)?;

    let jwt = handle_response(response).with_context(|| "Failed to handle login response")?;
    Ok(jwt)
}

pub fn report_types(access_token: &str) -> Result<response::ReportTypes> {
    let url = create_url(BASE_URL, ENDPOINT_REPORTTYPES);
    let request = reqwest::blocking::Client::new()
        .get(url)
        .query(&[("accesskey", access_token)]);
    let response = send(request)?;

    Ok(handle_response(response).with_context(|| "Failed to handle report-types response")?)
}

pub fn reports(access_token: &str, report_type: i64) -> Result<response::Reports> {
    let url = create_url(BASE_URL, ENDPOINT_REPORTS);
    let request = reqwest::blocking::Client::new().get(url).query(&[
        ("accesskey", access_token),
        ("id", &report_type.to_string()),
    ]);
    let response = send(request)?;

    Ok(handle_response(response).with_context(|| "Failed to handle reports response")?)
}

pub fn pull_all(access_token: &str) -> Result<response::All> {
    let url = create_url(BASE_URL, ENDPOINT_PULL_ALL);
    let request = reqwest::blocking::Client::new()
        .get(url)
        .query(&[("accesskey", access_token)]);
    let response = send(request)?;

    Ok(handle_response(response).with_context(|| "Failed to handle pull all response")?)
}

// pub fn _export_users(jwt: &str) -> Result<response::User> {
//     let url = create_url(BASE_URL, ENDPOINT_EXPORT_USERS);
//     let request = reqwest::blocking::Client::new()
//         .get(url)
//         .header(COOKIE, format!("_jwt={jwt}"));
//     let response = send(request)?;

//     Ok(handle_response(response).with_context(|| "Failed to handle export-users response")?)
// }

// pub fn _users(access_token: &str) -> Result<response::User> {
//     let url = create_url(BASE_URL, ENDPOINT_USERS);
//     let request = reqwest::blocking::Client::new()
//         .get(url)
//         .query(&[("accesskey", access_token)]);
//     let response = send(request)?;

//     Ok(handle_response(response).with_context(|| "Failed to handle users response")?)
// }

fn create_url(base: &str, endpoint: &str) -> Url {
    Url::parse(base).unwrap().join(endpoint).unwrap()
}

fn send(request: RequestBuilder) -> Result<Response> {
    let response = request.send()?;
    log::debug!("Response headers: {:#?}", &response);

    Ok(response)
}

fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T> {
    if !response.status().is_success() {
        bail!("Request failed with {}", response.status());
    }

    let response_text = response.text()?;
    let raw_response: Value =
        serde_json::from_str(&response_text).with_context(|| "Failed to parse JSON")?;
    log::debug!("Response text: {:#?}", raw_response);

    if !raw_response.get("success").unwrap().as_bool().unwrap() {
        bail!("Request failed {}", raw_response.get("errors").unwrap());
    }

    let response: SuccessResponse<T> = serde_json::from_str(&response_text)
        .with_context(|| format!("Failed to parse JSON to {}", type_name::<T>()))?;
    Ok(response.data)
}
