use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct WangSu {
    name: String,
}

impl Plugin for WangSu {
    fn check(
        &self,
        content: &str,
        _status: reqwest::StatusCode,
        headers: &reqwest::header::HeaderMap,
    ) -> Result<bool> {
        if self.match_content(content)? || self.has_header_key(headers)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for WangSu {
    fn default() -> Self {
        Self::new()
    }
}

impl WangSu {
    pub fn new() -> Self {
        WangSu {
            name: "Wangsu (wangsu.com)".to_string(),
        }
    }

    pub fn match_content(&self, content: &str) -> Result<bool> {
        let pattern = Regex::new(r"waf-interTip.jpg").context("wangsu new regex error")?;
        Ok(pattern.is_match(content))
    }

    pub fn has_header_key(&self, headers: &reqwest::header::HeaderMap) -> Result<bool> {
        let val = headers.get("x-ss-request-id");
        match val {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::plugins::wangsu::WangSu;

    fn get_html_content() -> String {
        fs::read_to_string("tests/html/wangsu.html").unwrap()
    }

    #[test]
    fn test_match_content() {
        let content = get_html_content();
        let waf = WangSu::new();

        let res = waf.match_content(&content).unwrap();
        assert!(res)
    }
}
