pub mod safeline;
use anyhow::Result;

pub trait Plugin {
    fn check(&self, content: &str) -> Result<bool>;
    fn name(&self) -> String;
}
