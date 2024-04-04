use super::Plugin;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use regex::Regex;
use scraper::{Html, Selector};
use sha2::Digest;
use sha2::Sha256;

// safeline waf img sha256
const SHAVALUE: &str = "81f620c6a25e7c0f51d78c891dba694884c8345b63a76ecf97b0885a1d9c6b26";

#[derive(Debug)]
pub struct Safeline {
    name: String,
}

impl Plugin for Safeline {
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

impl Default for Safeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Safeline {
    pub fn new() -> Self {
        Safeline {
            name: "Safeline (Chaitin Tech.)".to_string(),
        }
    }

    fn match_image(&self, content: &str) -> Result<bool> {
        let document = Html::parse_document(content);
        let image_selector =
            Selector::parse("img[alt=拦截]").map_err(|err| anyhow!("parse html error {}", err))?;
        for element in document.select(&image_selector) {
            let mut hasher = Sha256::new();
            if let Some(img_src) = element.value().attr("src") {
                hasher.update(img_src);
                let res = format!("{:x}", hasher.finalize());
                if res == SHAVALUE {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    fn match_content(&self, content: &str) -> Result<bool> {
        let pattern = Regex::new(r"safeline|<!-- event_id:").context("safeline new regex error")?;
        Ok(pattern.is_match(content))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::Safeline;

    fn get_html_content() -> String {
        fs::read_to_string("tests/html/safeline.html").unwrap()
    }

    #[test]
    fn test_match_image() {
        let content = get_html_content();
        let safeline = Safeline::new();

        let res = safeline.match_image(&content).unwrap();
        assert!(res)
    }

    #[test]
    fn test_match_content() {
        let content = get_html_content();
        let safeline = Safeline::new();

        let res = safeline.match_content(&content).unwrap();
        assert!(res)
    }
}
