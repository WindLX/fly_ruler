use fly_ruler_core::core::CoreInitCfg;
use fly_ruler_utils::error::FrError;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub struct ConfigManager {
    core_init_path: PathBuf,
}

impl ConfigManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let core_init_path = path.as_ref().join("core_init.toml");

        Self { core_init_path }
    }

    pub fn load_core_init(&self) -> Result<CoreInitCfg, FrError> {
        let core_init = read_to_string(&self.core_init_path).map_err(|e| FrError::Io(e))?;
        let core_init: CoreInitCfg =
            toml::from_str(&core_init).map_err(|e| FrError::Cfg(e.to_string()))?;
        Ok(core_init)
    }
}
