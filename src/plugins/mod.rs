pub mod aliyundun;
pub mod bt;
pub mod chuangyu;
pub mod g01;
pub mod huawei;
pub mod safe3;
pub mod safedog;
pub mod safeline;
pub mod tencent;
pub mod wangzhanbao;
pub mod wordfence;
pub mod wts;

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
