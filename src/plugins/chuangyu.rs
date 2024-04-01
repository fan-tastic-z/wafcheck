use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct ChuangYuDun {
    name: String,
}

impl Plugin for ChuangYuDun {
    fn check(&self, content: &str, _status: reqwest::StatusCode) -> Result<bool> {
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

impl Default for ChuangYuDun {
    fn default() -> Self {
        Self::new()
    }
}

impl ChuangYuDun {
    pub fn new() -> Self {
        ChuangYuDun {
            name: "Chuang Yu Shield (Yunaq)".to_string(),
        }
    }

    fn match_content(&self, content: &str) -> Result<bool> {
        let patterns = [
            r"help\.365cyd\.com/cyd\-error\-help.html\?code=403",
            r"已被创宇盾拦截",
            r"Knownsec CloudWAF:",
            r"请登录知道创宇云防御",
        ];
        let combined_pattern = patterns.join("|");
        let pattern = Regex::new(&combined_pattern).context("chuangyudun new regex error")?;
        Ok(pattern.is_match(content))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::ChuangYuDun;

    fn get_html_content(name: &str) -> String {
        fs::read_to_string(name).unwrap()
    }

    #[test]
    fn test_match_content() {
        let content = get_html_content("tests/html/chuangyudun.html");

        let waf = ChuangYuDun::new();

        let res = waf.match_content(&content).unwrap();
        assert!(res)
    }

    #[test]
    fn test_match_content2() {
        let content = get_html_content("tests/html/chuangyudun2.html");

        let waf = ChuangYuDun::new();

        let res = waf.match_content(&content).unwrap();
        assert!(res)
    }
}
