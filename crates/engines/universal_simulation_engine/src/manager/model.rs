use super::{plugin::AsPluginManager, PluginManager};
use fly_ruler_plugin::{AerodynamicModel, AsPlugin, PluginState};
use std::path::Path;
use tracing::{event, Level};

pub struct ModelManager {
    inner: PluginManager<AerodynamicModel>,
}

impl ModelManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let inner =
            PluginManager::<AerodynamicModel>::new(path, |path| AerodynamicModel::new(path));
        Self { inner }
    }

    pub fn get_model(&self, model_id: uuid::Uuid) -> Option<&AerodynamicModel> {
        match self.inner.plugin(model_id) {
            Some(model) => {
                if model.state() == PluginState::Enable {
                    Some(model)
                } else {
                    event!(Level::WARN, "model {} is not enabled", model.info().name);
                    None
                }
            }
            None => {
                event!(Level::WARN, "model {} not found", model_id);
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
