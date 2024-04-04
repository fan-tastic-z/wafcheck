use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct NinjaFirewall {
    name: String,
}

impl Plugin for NinjaFirewall {
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

impl Default for NinjaFirewall {
    fn default() -> Self {
        Self::new()
    }
}

impl NinjaFirewall {
    pub fn new() -> Self {
        NinjaFirewall {
            name: "NinjaFirewall (NinTechNet)".to_string(),
        }
    }

    fn match_content(&self, content: &str) -> Result<bool> {
        let patterns = [
            r"<title>NinjaFirewall",
            r"For security reasons?.{0,10}?it was blocked and logged",
        ];
        let combined_pattern = patterns.join("|");
        let pattern = Regex::new(&combined_pattern).context("ninjia firewalld new regex error")?;
        Ok(pattern.is_match(content))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::plugins::ninjafirewall::NinjaFirewall;

    fn get_html_content() -> String {
        fs::read_to_string("tests/html/ninjafirewall.html").unwrap()
    }

    #[test]
    fn test_match_content() {
        let content = get_html_content();
        let waf = NinjaFirewall::new();

        let res = waf.match_content(&content).unwrap();
        assert!(res)
    }
}
