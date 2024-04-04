use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct AwsWaf {
    name: String,
}

impl Plugin for AwsWaf {
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

impl Default for AwsWaf {
    fn default() -> Self {
        Self::new()
    }
}

impl AwsWaf {
    pub fn new() -> Self {
        AwsWaf {
            name: "AWS Elastic Load Balancer (Amazon)".to_string(),
        }
    }

    pub fn match_header(&self, headers: &reqwest::header::HeaderMap) -> Result<bool> {
        let val = headers.get("server");
        match val {
            Some(val) => {
                let pattern = Regex::new(r"awselb").context("aws waf new regex error")?;
                let contenet = val.to_str()?;
                Ok(pattern.is_match(contenet))
            }
            None => Ok(false),
        }
    }

    pub fn match_set_cookie(&self, headers: &reqwest::header::HeaderMap) -> Result<bool> {
        let set_cookie = headers.get_all(reqwest::header::SET_COOKIE);
        if let Some(v) = set_cookie.into_iter().next() {
            let patterns = [r"awsalbtg", r"awsalbtgcors"];
            let combined_pattern = patterns.join("|");
            let pattern = Regex::new(&combined_pattern).context("aws waf new regex error")?;
            return Ok(pattern.is_match(v.to_str()?));
        }
        Ok(false)
    }
}
