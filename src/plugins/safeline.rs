use crypto::{digest::Digest, md5::Md5};
use regex::Regex;
use scraper::{Html, Selector};

use super::Plugin;

// safeline waf img md5
const IMAGEMD5: &str = "a2c06ca7f40785c8ea28aab8756e4dea";

pub struct Safeline {
    name: String,
}

impl Plugin for Safeline {
    fn check(&self, content: &str) -> bool {
        self.is_match(content)
    }

    fn name(&self) -> String {
        return self.name.clone();
    }
}

impl Safeline {
    pub fn new() -> Self {
        Safeline {
            name: "Safeline (Chaitin Tech.)".to_string(),
        }
    }

    fn is_match(&self, content: &str) -> bool {
        if self.match_image(content) {
            return true;
        } else if self.match_content(content) {
            return true;
        } else {
            return false;
        }
    }

    fn match_image(&self, content: &str) -> bool {
        let document = Html::parse_document(content);
        let image_selector = Selector::parse("img[alt=拦截]").unwrap();
        for element in  document.select(&image_selector) {
            let mut hasher = Md5::new();
            hasher.input_str(element.value().attr("src").unwrap());
            if hasher.result_str() == IMAGEMD5 {
                return true;
            }
        }
        return false;
    }

    fn match_content(&self, content: &str) -> bool {
        let pattern = Regex::new(r"safeline|<!-- event_id:").unwrap();
        pattern.is_match(content)
    }
}