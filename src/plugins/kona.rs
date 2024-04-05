use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct Kona {
    name: String,
}

impl Plugin for Kona {
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

impl Default for Kona {
    fn default() -> Self {
        Self::new()
    }
}

impl Kona {
    pub fn new() -> Self {
        Kona {
            name: "Kona SiteDefender (Akamai)".to_string(),
        }
    }

    pub fn match_header(&self, headers: &reqwest::header::HeaderMap) -> Result<bool> {
        let val = headers.get("server");
        match val {
            Some(val) => {
                let patterns = [r"AkamaiGHost", r"AkamaiNetStorage", r"Akamai"];
                let combined_pattern = patterns.join("|");
                let pattern =
                    Regex::new(&combined_pattern).context("kona SiteDefender new regex error")?;
                let contenet = val.to_str()?;
                Ok(pattern.is_match(contenet))
            }
            None => Ok(false),
        }
    }
}
