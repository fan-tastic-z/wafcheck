use anyhow::{Context, Result};
use regex::Regex;

use super::Plugin;

#[derive(Debug)]
pub struct WangZhanBao {
    name: String,
}

impl Plugin for WangZhanBao {
    fn check(
        &self,
        content: &str,
        status: reqwest::StatusCode,
        headers: &reqwest::header::HeaderMap,
    ) -> Result<bool> {
        if self.match_content(content)?
            || self.match_header(headers)?
            || self.match_status(status)
            || self.match_set_cookie(headers)?
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for WangZhanBao {
    fn default() -> Self {
        Self::new()
    }
}

impl WangZhanBao {
    pub fn new() -> Self {
        WangZhanBao {
            name: "360WangZhanBao (360 Technologies)".to_string(),
        }
    }

    pub fn match_header(&self, headers: &reqwest::header::HeaderMap) -> Result<bool> {
        let val = headers.get("wzws-ray");
        match val {
            Some(val) => {
                let pattern = Regex::new(r"waf").context("360WangZhanBao new regex error")?;
                let contenet = val.to_str()?;
                Ok(pattern.is_match(contenet))
            }
            None => Ok(false),
        }
    }

    pub fn match_set_cookie(&self, headers: &reqwest::header::HeaderMap) -> Result<bool> {
        let set_cookie = headers.get_all(reqwest::header::SET_COOKIE);
        if let Some(v) = set_cookie.into_iter().next() {
            let pattern =
                Regex::new(r"wzws_sessionid").context("360WangZhanBao new regex error")?;
            return Ok(pattern.is_match(v.to_str()?));
        }
        Ok(false)
    }

    pub fn match_content(&self, content: &str) -> Result<bool> {
        let pattern = Regex::new(r"已被云防护拦截").context("360WangZhanBao new regex error")?;
        Ok(pattern.is_match(content))
    }

    pub fn match_status(&self, status: reqwest::StatusCode) -> bool {
        status == 493
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::plugins::wangzhanbao::WangZhanBao;

    fn get_html_content() -> String {
        fs::read_to_string("tests/html/wangzhanbao.html").unwrap()
    }

    #[test]
    fn test_match_content() {
        let content = get_html_content();
        let waf = WangZhanBao::new();

        let res = waf.match_content(&content).unwrap();
        assert!(res)
    }
}
