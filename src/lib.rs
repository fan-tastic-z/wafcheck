use plugins::Plugin;

use crate::plugins::safeline::Safeline;

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

    pub fn run_check(&self, content: &str) -> Option<String> {
        for plugin in &self.plugins {
            let check = plugin.check(content);
            match check {
                Ok(is_match) => {
                    if is_match {
                        return Some(plugin.name());
                    }
                }
                Err(err) => {
                    eprintln!("try match plugin {} error {}", plugin.name(), err);
                }
            }
        }
        None
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
    register_plugin!(Safeline::new())
}