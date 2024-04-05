use plugins::Plugin;
use rayon::prelude::*;

use crate::plugins::{
    aliyundun::AliYunDun, awswaf::AwsWaf, bt::Bt, chuangyu::ChuangYuDun, cloudflare::CloudFlare,
    cloudfront::CloudFront, g01::G01, huawei::HuaWei, kona::Kona, ninjafirewall::NinjaFirewall,
    safe3::Safe3, safedog::SafeDog, safeline::Safeline, tencent::Tencent, wangzhanbao::WangZhanBao,
    wordfence::Wordfence, wts::WtsWaf,
};

pub mod help;
pub mod plugins;

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    fn register_plugin<T: Plugin + 'static>(&mut self, plugin: T) {
        self.plugins.push(Box::new(plugin));
    }

    pub fn support_check_wafs(&self) -> Vec<String> {
        self.plugins.iter().map(|i| i.name()).collect()
    }

    pub fn run_check(
        &self,
        content: &str,
        status: reqwest::StatusCode,
        headers: &reqwest::header::HeaderMap,
    ) -> Option<String> {
        let result: Option<String> = self
            .plugins
            .par_iter()
            .find_any(|plugin| {
                let check = plugin.check(content, status, headers);
                match check {
                    Ok(is_match) => is_match,
                    Err(err) => {
                        eprintln!("try match plugin {} error {}", plugin.name(), err);
                        false
                    }
                }
            })
            .map(|plugin| plugin.name());
        result
    }
}

macro_rules! register_plugin {
    ($($plugin:expr),*) => {
        {
            let mut plugin_manager = PluginManager::new();
            $(
                plugin_manager.register_plugin($plugin);
            )*
            plugin_manager
        }
    };
}

pub fn init() -> PluginManager {
    register_plugin!(
        Safeline::new(),
        AliYunDun::new(),
        Tencent::new(),
        ChuangYuDun::new(),
        HuaWei::new(),
        WangZhanBao::new(),
        G01::new(),
        WtsWaf::new(),
        Wordfence::new(),
        SafeDog::new(),
        Bt::new(),
        Safe3::new(),
        NinjaFirewall::new(),
        CloudFront::new(),
        AwsWaf::new(),
        Kona::new(),
        CloudFlare::new()
    )
}
