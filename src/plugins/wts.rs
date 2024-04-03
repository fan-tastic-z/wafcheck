use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct WtsWaf {
    name: String,
}

impl Plugin for WtsWaf {
    fn check(
        &self,
        content: &str,
        _status: reqwest::StatusCode,
        headers: &reqwest::header::HeaderMap,
    ) -> Result<bool> {
        if self.match_content(content)? || self.match_header(headers)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for WtsWaf {
    fn default() -> Self {
        Self::new()
    }
}

impl WtsWaf {
    pub fn new() -> Self {
        WtsWaf {
            name: "WTS-WAF (WTS)".to_string(),
        }
    }

    fn match_content(&self, content: &str) -> Result<bool> {
        let pattern = Regex::new(r"WTS-WAF").context("wts waf new regex error")?;
        Ok(pattern.is_match(content))
    }

    pub fn match_header(&self, headers: &reqwest::header::HeaderMap) -> Result<bool> {
        let val = headers.get("server");
        match val {
            Some(val) => {
                let pattern = Regex::new(r"wts").context("wts waf new regex error")?;
                let contenet = val.to_str()?;
                Ok(pattern.is_match(contenet))
            }
            None => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::plugins::wts::WtsWaf;

    fn get_html_content() -> String {
        fs::read_to_string("tests/html/wts_waf.html").unwrap()
    }

    #[test]
    fn test_match_content() {
        let content = get_html_content();
        let waf = WtsWaf::new();

        let res = waf.match_content(&content).unwrap();
        assert!(res)
    }
}
