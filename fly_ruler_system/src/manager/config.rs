use fly_ruler_core::core::CoreInitCfg;
use fly_ruler_utils::error::FrError;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub struct ConfigManager {
    core_path: PathBuf,
    sys_path: PathBuf,
}

impl ConfigManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let core_path = path.as_ref().join("core.toml");
        let sys_path = path.as_ref().join("sys.toml");

        Self {
            core_path,
            sys_path,
        }
    }

    fn load_cfg<C: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<C, FrError> {
        let str = read_to_string(path).map_err(|e| FrError::Io(e))?;
        let cfg: C = toml::from_str(&str).map_err(|e| FrError::Cfg(e.to_string()))?;
        Ok(cfg)
    }

    pub fn load_core_cfg(&self) -> Result<CoreInitCfg, FrError> {
        Self::load_cfg(&self.core_path)
    }

    // TODO SysCfg
    pub fn load_sys_cfg(&self) -> Result<CoreInitCfg, FrError> {
        Self::load_cfg(&self.sys_path)
    }
}
