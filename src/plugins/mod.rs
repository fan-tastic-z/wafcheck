pub mod aliyundun;
pub mod safeline;
pub mod tencent;

use anyhow::Result;

pub trait Plugin {
    fn check(&self, content: &str, status: reqwest::StatusCode) -> Result<bool>;
    fn name(&self) -> String;
}
