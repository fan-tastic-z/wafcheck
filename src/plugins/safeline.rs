use super::Plugin;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use crypto::{digest::Digest, md5::Md5};
use regex::Regex;
use scraper::{Html, Selector};

// safeline waf img md5
const IMAGEMD5: &str = "a2c06ca7f40785c8ea28aab8756e4dea";

#[derive(Debug)]
pub struct Safeline {
    name: String,
}

impl Plugin for Safeline {
    fn check(&self, content: &str) -> Result<bool> {
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
            let mut hasher = Md5::new();
            if let Some(img_src) = element.value().attr("src") {
                hasher.input_str(img_src);
                if hasher.result_str() == IMAGEMD5 {
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
