use anyhow::Context;
use anyhow::Result;
use regex::Regex;
use reqwest::StatusCode;

use super::Plugin;

#[derive(Debug)]
pub struct AliYunDun {
    name: String,
}

impl Plugin for AliYunDun {
    fn check(
        &self,
        content: &str,
        status: reqwest::StatusCode,
        _headers: &reqwest::header::HeaderMap,
    ) -> Result<bool> {
        if self.match_content(content)? && self.match_status(status) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for AliYunDun {
    fn default() -> Self {
        Self::new()
    }
}

impl AliYunDun {
    pub fn new() -> Self {
        AliYunDun {
            name: "AliYunDun (Alibaba Cloud Computing)".to_string(),
        }
    }

    fn match_content(&self, content: &str) -> Result<bool> {
        let pattern = Regex::new(r"errors.aliyun.com").context("aliyundun new regex error")?;
        Ok(pattern.is_match(content))
    }

    fn match_status(&self, status: reqwest::StatusCode) -> bool {
        status == StatusCode::METHOD_NOT_ALLOWED
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::AliYunDun;

    fn get_html_content() -> String {
        fs::read_to_string("tests/html/aliyundun.html").unwrap()
    }

    #[test]
    fn test_match_content() {
        let content = get_html_content();
        let aliuyundun = AliYunDun::new();

        let res = aliuyundun.match_content(&content).unwrap();

        assert!(res)
    }
}
