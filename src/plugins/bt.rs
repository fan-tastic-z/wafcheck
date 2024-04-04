use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct Bt {
    name: String,
}

impl Plugin for Bt {
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

impl Default for Bt {
    fn default() -> Self {
        Self::new()
    }
}

impl Bt {
    pub fn new() -> Self {
        Bt {
            name: "宝塔网站防火墙免费版".to_string(),
        }
    }

    fn match_content(&self, content: &str) -> Result<bool> {
        let patterns = [r"宝塔网站防火墙免费版", r"www\.bt\.cn\/bbs"];
        let combined_pattern = patterns.join("|");
        let pattern = Regex::new(&combined_pattern).context("bt new regex error")?;
        Ok(pattern.is_match(content))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::plugins::bt::Bt;

    fn get_html_content(name: &str) -> String {
        fs::read_to_string(name).unwrap()
    }

    #[test]
    fn test_match_content() {
        let content = get_html_content("tests/html/baota.html");

        let waf = Bt::new();

        let res = waf.match_content(&content).unwrap();
        assert!(res)
    }
}
