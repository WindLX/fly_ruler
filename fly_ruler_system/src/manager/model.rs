use super::{plugin::AsPluginManager, PluginManager};
use fly_ruler_plugin::{AerodynamicModel, AsPlugin, PluginState};
use log::warn;
use std::path::Path;

pub struct ModelManager {
    inner: PluginManager<AerodynamicModel>,
}

impl ModelManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let inner =
            PluginManager::<AerodynamicModel>::new(path, |path| AerodynamicModel::new(path));
        Self { inner }
    }

    pub fn get_model(&self, index: usize) -> Option<&AerodynamicModel> {
        match self.inner.plugin(index) {
            Some(model) => {
                if model.state() == PluginState::Enable {
                    Some(model)
                } else {
                    warn!("model {} is not enabled", model.info().name);
                    None
                }
            }
            None => {
                warn!("model {} not found", index);
                None
            }
        }
    }
}

impl AsPluginManager<AerodynamicModel> for ModelManager {
    fn plugin_manager(&self) -> &PluginManager<AerodynamicModel> {
        &self.inner
    }

    fn plugin_manager_mut(&mut self) -> &mut PluginManager<AerodynamicModel> {
        &mut self.inner
    }
}
