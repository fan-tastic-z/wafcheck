pub mod aliyundun;
pub mod chuangyu;
pub mod huawei;
pub mod safeline;
pub mod tencent;
pub mod wangzhanbao;

use anyhow::Result;

pub trait Plugin {
    fn check(
        &self,
        content: &str,
        status: reqwest::StatusCode,
        headers: &reqwest::header::HeaderMap,
    ) -> Result<bool>;
    fn name(&self) -> String;
}
