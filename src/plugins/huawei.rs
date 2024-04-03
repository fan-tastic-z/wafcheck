use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct HuaWei {
    name: String,
}

impl Plugin for HuaWei {
    fn check(
        &self,
        content: &str,
        _status: reqwest::StatusCode,
        _headers: &reqwest::header::HeaderMap,
    ) -> Result<bool> {
        if self.match_content(content)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for HuaWei {
    fn default() -> Self {
        Self::new()
    }
}

impl HuaWei {
    pub fn new() -> Self {
        HuaWei {
            name: "Huawei Cloud WAF (Huawei)".to_string(),
        }
    }

    fn match_content(&self, content: &str) -> Result<bool> {
        let pattern = Regex::new(r"HuaweiCloudWAF").context("Huawei new regex error")?;
        Ok(pattern.is_match(content))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::HuaWei;

    fn get_html_content() -> String {
        fs::read_to_string("tests/html/huawei.html").unwrap()
    }

    #[test]
    pub fn test_match_content() {
        let content = get_html_content();
        let waf = HuaWei::new();

        let res = waf.match_content(&content).unwrap();
        assert!(res)
    }
}
