use crate::model::AerodynamicModel;
use crate::plugin::{IsPlugin, PluginError, PluginInfo, PluginState};
use fly_ruler_utils::error::FrError;
use log::{debug, warn};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum PluginType {
    System,
    Controller,
    Model,
}

pub struct PluginManager {
    models: HashMap<usize, AerodynamicModel>,
}

fn load_plugins<Pl: IsPlugin>(
    dir: PathBuf,
    loader: Box<dyn Fn(&Path) -> Result<Pl, PluginError>>,
) -> HashMap<usize, Pl> {
    let mut plugins = HashMap::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for (idx, entry) in entries.into_iter().filter_map(|e| e.ok()).enumerate() {
            if entry.file_type().is_ok_and(|f| f.is_dir()) {
                let sub_dir = entry.path();
                debug!("find plugin directory: {}", sub_dir.display());
                let plugin = loader(&sub_dir);
                match plugin {
                    Ok(p) => {
                        plugins.insert(idx, p);
                    }
                    Err(e) => {
                        warn!("{}", e);
                    }
                }
            }
        }
    }

    plugins
}

impl PluginManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let model_path = path.as_ref().join("model");

        let models = load_plugins(model_path, Box::new(|dir| AerodynamicModel::new(dir)));

        Self { models }
    }

    pub fn get_infos(&self) -> Vec<HashMap<usize, PluginInfo>> {
        let mut models = HashMap::new();

        for (idx, model) in &self.models {
            let model_info = model.info().clone();
            models.insert(*idx, model_info);
        }
        vec![models]
    }

    pub fn install(
        &mut self,
        plugin_type: PluginType,
        index: &usize,
        args: &[impl ToString],
    ) -> Result<(), FrError> {
        match plugin_type {
            PluginType::Model => match self.models.get_mut(&index) {
                Some(model) => {
                    if model.state() == &PluginState::Uninstalled {
                        match model.plugin().install(args) {
                            Ok(Ok(())) => {
                                model.plugin_mut().set_state(PluginState::Installed);
                                Ok(())
                            }
                            Ok(Err(e)) => {
                                model.plugin_mut().set_state(PluginState::Failed);
                                warn!("{}", e);
                                Ok(())
                            }
                            Err(e) => {
                                model.plugin_mut().set_state(PluginState::Failed);
                                Err(FrError::Plugin(e))
                            }
                        }
                    } else {
                        warn!("model {} can't be installed", model.info().name);
                        Ok(())
                    }
                }
                None => {
                    warn!("model {} not found", index);
                    Ok(())
                }
            },
            PluginType::System => todo!(),
            PluginType::Controller => todo!(),
        }
    }

    pub fn uninstall(
        &mut self,
        plugin_type: PluginType,
        index: &usize,
        args: &[impl ToString],
    ) -> Result<(), FrError> {
        match plugin_type {
            PluginType::Model => match self.models.get_mut(&index) {
                Some(model) => {
                    if model.state() == &PluginState::Installed {
                        match model.plugin().uninstall(args) {
                            Ok(Ok(())) => {
                                model.plugin_mut().set_state(PluginState::Uninstalled);
                                Ok(())
                            }
                            Ok(Err(e)) => {
                                model.plugin_mut().set_state(PluginState::Failed);
                                warn!("{}", e);
                                Ok(())
                            }
                            Err(e) => {
                                model.plugin_mut().set_state(PluginState::Failed);
                                Err(FrError::Plugin(e))
                            }
                        }
                    } else {
                        // warn!("Model {} can't be installed", model.info().name);
                        Ok(())
                    }
                }
                None => {
                    warn!("model {} not found", index);
                    Ok(())
                }
            },
            PluginType::System => todo!(),
            PluginType::Controller => todo!(),
        }
    }

    pub fn uninstall_all(&mut self) -> Result<(), FrError> {
        let keys = self.models.keys().map(|k| k.clone()).collect::<Vec<_>>();
        for idx in keys {
            self.uninstall(PluginType::Model, &idx, &[""])?;
        }
        Ok(())
    }

    pub fn get_model(&mut self, index: &usize) -> Option<&AerodynamicModel> {
        match self.models.get_mut(&index) {
            Some(model) => {
                if model.state() == &PluginState::Installed {
                    Some(model)
                } else {
                    warn!("M=model {} must be installed", model.info().name);
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
