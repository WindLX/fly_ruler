use crate::plugin::{IsPlugin, PluginError, PluginInfo, PluginState};
use crate::{model::AerodynamicModel, plugin::Plugin};
use fly_ruler_utils::error::FrError;
use log::{debug, warn};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy)]
pub enum PluginType {
    System,
    Controller,
    Model,
}

pub struct PluginManager {
    models: HashMap<usize, AerodynamicModel>,
    systems: HashMap<usize, Plugin>,
    controllers: HashMap<usize, Plugin>,
}

fn load_plugins<Pl: IsPlugin>(
    dir: PathBuf,
    loader: Box<dyn Fn(&Path) -> Result<Pl, PluginError>>,
) -> HashMap<usize, Pl> {
    let mut plugins = HashMap::new();
    for (idx, entry) in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .enumerate()
    {
        if entry.file_type().is_dir() {
            let sub_dir = entry.path();
            debug!("find plugin directory: {}", sub_dir.display());
            let plugin = loader(sub_dir);
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
    plugins
}

impl PluginManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let model_path = path.as_ref().join("model");
        let system_path = path.as_ref().join("system");
        let controller_path = path.as_ref().join("controller");

        let models = load_plugins(model_path, Box::new(|dir| AerodynamicModel::new(dir)));
        let systems = load_plugins(system_path, Box::new(|dir| Plugin::new(dir)));
        let controllers = load_plugins(controller_path, Box::new(|dir| Plugin::new(dir)));

        Self {
            models,
            systems,
            controllers,
        }
    }

    pub fn get_infos(&self) -> Vec<HashMap<usize, PluginInfo>> {
        let mut models = HashMap::new();
        let mut systems = HashMap::new();
        let mut controllers = HashMap::new();

        for (idx, model) in &self.models {
            let model_info = model.info().clone();
            models.insert(*idx, model_info);
        }
        for (idx, system) in &self.systems {
            let system_info = system.info().clone();
            systems.insert(*idx, system_info);
        }
        for (idx, controller) in &self.controllers {
            let controller_info = controller.info().clone();
            controllers.insert(*idx, controller_info);
        }
        vec![models, systems, controllers]
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
                        warn!("Model {} can't be installed", model.info().name);
                        Ok(())
                    }
                }
                None => {
                    warn!("Model {} not found", index);
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
                    warn!("Model {} not found", index);
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
        let keys = self.systems.keys().map(|k| k.clone()).collect::<Vec<_>>();
        for idx in keys {
            self.uninstall(PluginType::System, &idx, &[""])?;
        }
        let keys = self
            .controllers
            .keys()
            .map(|k| k.clone())
            .collect::<Vec<_>>();
        for idx in keys {
            self.uninstall(PluginType::Controller, &idx, &[""])?;
        }
        Ok(())
    }

    pub fn get_model(&mut self, index: &usize) -> Option<&AerodynamicModel> {
        match self.models.get_mut(&index) {
            Some(model) => {
                if model.state() == &PluginState::Installed {
                    Some(model)
                } else {
                    warn!("Model {} must be installed", model.info().name);
                    None
                }
            }
            None => {
                warn!("Model {} not found", index);
                None
            }
        }
    }
}
