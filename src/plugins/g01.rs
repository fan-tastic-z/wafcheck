use super::Plugin;
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use scraper::{Html, Selector};
use sha2::{Digest, Sha256};

const SHAVALUE: &str = "1d48caed40d73ddf5f1f000e8589cece22e43dc794788d6a4dec4b300e23835a";

#[derive(Debug)]
pub struct G01 {
    name: String,
}

impl Plugin for G01 {
    fn check(
        &self,
        content: &str,
        _status: reqwest::StatusCode,
        _headers: &reqwest::header::HeaderMap,
    ) -> Result<bool> {
        if self.match_content(content)? || self.match_image(content)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for G01 {
    fn default() -> Self {
        Self::new()
    }
}

impl G01 {
    pub fn new() -> Self {
        G01 {
            name: "G01".to_string(),
        }
    }

    fn match_content(&self, content: &str) -> Result<bool> {
        let patterns = [r"网防G01", r"yunsuo"];
        let combined_pattern = patterns.join("|");
        let pattern = Regex::new(&combined_pattern).context("G01 new regex error")?;
        Ok(pattern.is_match(content))
    }

    fn match_image(&self, content: &str) -> Result<bool> {
        let document = Html::parse_document(content);
        let image_selector =
            Selector::parse("img.logo").map_err(|err| anyhow!("parse html error {}", err))?;
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
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::plugins::g01::G01;

    fn get_html_content() -> String {
        fs::read_to_string("tests/html/g01.html").unwrap()
    }

    #[test]
    fn test_match_image() {
        let content = get_html_content();
        let waf = G01::new();

        let res = waf.match_image(&content).unwrap();
        assert!(res)
    }

    #[test]
    fn test_match_content() {
        let content = get_html_content();
        let waf = G01::new();

        let res = waf.match_content(&content).unwrap();
        assert!(res)
    }
}
