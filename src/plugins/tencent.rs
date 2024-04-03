use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use regex::Regex;
use scraper::{Html, Selector};

use super::Plugin;

const IMGSRC: &str = "https://imgcache.qq.com/qcloud/security/static/imgs/attackIntercept.svg";

#[derive(Debug)]
pub struct Tencent {
    name: String,
}

impl Plugin for Tencent {
    fn check(
        &self,
        content: &str,
        _status: reqwest::StatusCode,
        _headers: &reqwest::header::HeaderMap,
    ) -> Result<bool> {
        if self.match_image(content)? || self.match_content(content)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for Tencent {
    fn default() -> Self {
        Self::new()
    }
}

impl Tencent {
    pub fn new() -> Self {
        Tencent {
            name: "Tencent Cloud Firewall (Tencent Technologies)".to_string(),
        }
    }

    fn match_image(&self, content: &str) -> Result<bool> {
        let document = Html::parse_document(content);
        let image_selector = Selector::parse("div.accessDenySvg img")
            .map_err(|err| anyhow!("parse html error {}", err))?;
        if let Some(img) = document.select(&image_selector).next() {
            if let Some(img_src) = img.value().attr("src") {
                if img_src == IMGSRC {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    fn match_content(&self, content: &str) -> Result<bool> {
        let pattern = Regex::new(r"imgcache.qq.com").context("tencent new regex error")?;
        Ok(pattern.is_match(content))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::plugins::tencent::Tencent;

    fn get_html_content() -> String {
        fs::read_to_string("tests/html/tencent.html").unwrap()
    }

    #[test]
    fn test_match_image() {
        let content = get_html_content();
        let tencent = Tencent::new();

        let res = tencent.match_image(&content).unwrap();
        assert!(res)
    }

    #[test]
    fn test_match_content() {
        let content = get_html_content();
        let tencent = Tencent::new();

        let res = tencent.match_content(&content).unwrap();
        assert!(res)
    }
}
