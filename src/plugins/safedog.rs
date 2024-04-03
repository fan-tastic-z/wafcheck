use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct SafeDog {
    name: String,
}

impl Plugin for SafeDog {
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

impl Default for SafeDog {
    fn default() -> Self {
        Self::new()
    }
}

impl SafeDog {
    pub fn new() -> Self {
        SafeDog {
            name: "Safedog (SafeDog)".to_string(),
        }
    }
    fn match_content(&self, content: &str) -> Result<bool> {
        let patterns = [
            r"www\.safedog\.cn",
            r"security\.safedog\.cn",
            r"请登录安全狗",
        ];
        let combined_pattern = patterns.join("|");
        let pattern = Regex::new(&combined_pattern).context("safedog new regex error")?;
        Ok(pattern.is_match(content))
    }

    pub fn match_header(&self, headers: &reqwest::header::HeaderMap) -> Result<bool> {
        let val = headers.get("x-powerd-by");
        match val {
            Some(val) => {
                let pattern = Regex::new(r"waf").context("safedog new regex error")?;
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

    use crate::plugins::safedog::SafeDog;

    fn get_html_content() -> String {
        fs::read_to_string("tests/html/safedog.html").unwrap()
    }

    #[test]
    fn test_match_content() {
        let content = get_html_content();
        let waf = SafeDog::new();

        let res = waf.match_content(&content).unwrap();
        assert!(res)
    }
}
