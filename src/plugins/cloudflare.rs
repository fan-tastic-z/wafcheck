use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct CloudFlare {
    name: String,
}

impl Plugin for CloudFlare {
    fn check(
        &self,
        _content: &str,
        _status: reqwest::StatusCode,
        headers: &reqwest::header::HeaderMap,
    ) -> Result<bool> {
        if self.match_header(headers)? || self.match_set_cookie(headers)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for CloudFlare {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudFlare {
    pub fn new() -> Self {
        CloudFlare {
            name: "Cloudflare (Cloudflare Inc.)".to_string(),
        }
    }
    pub fn match_header(&self, headers: &reqwest::header::HeaderMap) -> Result<bool> {
        let val = headers.get("server");
        match val {
            Some(val) => {
                let pattern = Regex::new(r"cloudflare").context("aws waf new regex error")?;
                let contenet = val.to_str()?;
                Ok(pattern.is_match(contenet))
            }
            None => Ok(false),
        }
    }

    pub fn match_set_cookie(&self, headers: &reqwest::header::HeaderMap) -> Result<bool> {
        let set_cookie = headers.get_all(reqwest::header::SET_COOKIE);
        if let Some(v) = set_cookie.into_iter().next() {
            let pattern = Regex::new(r"cf_bm").context("cloudflare new regex error")?;
            let contenet = v.to_str()?;
            return Ok(pattern.is_match(contenet));
        }
        Ok(false)
    }
}
