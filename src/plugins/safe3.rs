use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct Safe3 {
    name: String,
}

impl Plugin for Safe3 {
    fn check(
        &self,
        _content: &str,
        _status: reqwest::StatusCode,
        headers: &reqwest::header::HeaderMap,
    ) -> Result<bool> {
        if self.match_header(headers)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for Safe3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Safe3 {
    pub fn new() -> Self {
        Safe3 {
            name: "Safe3 Web Firewall (Safe3)".to_string(),
        }
    }

    pub fn match_header(&self, headers: &reqwest::header::HeaderMap) -> Result<bool> {
        let power_by = headers.get("x-powerd-by");

        if let Some(x_power_by) = power_by {
            let pattern = Regex::new(r"Safe3WAF").context("safe3 new regex error")?;
            let contenet = x_power_by.to_str()?;
            if pattern.is_match(contenet) {
                return Ok(true);
            }
        }

        let server = headers.get("server");

        if let Some(server) = server {
            let pattern = Regex::new(r"Safe3 Web Firewall").context("safe3 new regex error")?;
            let contenet = server.to_str()?;
            if pattern.is_match(contenet) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
