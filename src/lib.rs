use plugins::Plugin;

use crate::plugins::safeline::Safeline;

pub mod plugins;

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>
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

    pub fn run_check(&self, content: &str) -> Option<String> {
        for plugin in &self.plugins {
            let check = plugin.check(content);
            if check {
                return Some(plugin.name());
            }
        }
        return None;
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
        Safeline::new()
    )
}