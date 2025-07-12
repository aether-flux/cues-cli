use colored::Colorize;
use keyring::Entry;
use chrono::{DateTime, Local};
use serde_json::json;

use crate::utils::display_format::log_err;

use super::api::post_refresh;

#[derive(Debug)]
pub struct AuthStore {
    pub access: Entry,
    pub refresh: Entry,
}

pub fn jwt_expired (expiry: &str) -> bool {
    match DateTime::parse_from_rfc3339(expiry) {
        Ok(ex) => ex < Local::now(),
        Err(_) => true,
    }
}

pub async fn refresh_access_token (r: &str) -> Result<String, Box<dyn std::error::Error>> {
    let payload = json!({
        "refresh_token": r,
    });

    let res = post_refresh(&payload).await?;

    if let Some(t) = res.get("accessToken") {
        return Ok(t.to_string());
    } else {
        println!();
        log_err(res);
        return Err(Box::from(""));
    }
}
