use super::ffi::{FrPluginInstallHook, FrPluginUninstallHook};
use fly_ruler_utils::FrError;
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::Path};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
}

impl PluginInfo {
    pub fn load<P: AsRef<Path>>(path: P) -> Option<PluginInfo> {
        let content = read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }
}

pub trait Plugin {
    fn info(&self) -> &PluginInfo;

    fn register_logger(&self) -> Result<(), Box<dyn PluginError>>;

    fn get_install_hook(&self) -> Result<FrPluginInstallHook, Box<dyn PluginError>>;

    fn get_uninstall_hook(&self) -> Result<FrPluginUninstallHook, Box<dyn PluginError>>;
}

pub trait PluginError: FrError {
    fn info(&self) -> &PluginInfo;
}
