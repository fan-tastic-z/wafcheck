use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct DynamicWeb {
    name: String,
}

impl Plugin for DynamicWeb {
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

impl Default for DynamicWeb {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicWeb {
    pub fn new() -> Self {
        DynamicWeb {
            name: "DynamicWeb Injection Check (DynamicWeb)".to_string(),
        }
    }
    pub fn match_header(&self, headers: &reqwest::header::HeaderMap) -> Result<bool> {
        let val = headers.get("x-404-status-by");
        match val {
            Some(val) => {
                let pattern = Regex::new(r"dw.inj.check").context("DynamicWeb new regex error")?;
                let contenet = val.to_str()?;
                Ok(pattern.is_match(contenet))
            }
            None => Ok(false),
        }
    }
}
