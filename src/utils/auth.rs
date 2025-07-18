use colored::Colorize;
use keyring::Entry;
use chrono::{DateTime, Local};
use serde_json::json;

use crate::utils::display_format::log_err;

use super::api::post_refresh;

// Storing access and refresh tokens in a keyring
#[derive(Debug)]
pub struct AuthStore {
    pub access: Entry,
    pub refresh: Entry,
}

// Check if existing JWT is expired
pub fn jwt_expired (expiry: &str) -> bool {
    match DateTime::parse_from_rfc3339(expiry) {
        Ok(ex) => ex < Local::now(),
        Err(_) => true,
    }
}

// Provide a refresh token and this function creates and returns a new access token and returns
pub async fn refresh_access_token (r: &str) -> Result<String, Box<dyn std::error::Error>> {
    let payload = json!({
        "refresh_token": r,
    });

    // API call
    let res = post_refresh(&payload).await?;

    // Extracting data from response
    if let Some(t) = res.get("accessToken") {
        // println!("New token in helper: {}", t);
        let token = t.as_str().unwrap_or("").trim_matches('"');
        return Ok(token.to_string());
    } else {
        println!();
        log_err(res);
        return Err(Box::from(""));
    }
}
