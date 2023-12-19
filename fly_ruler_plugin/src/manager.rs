use crate::plugin::{IsPlugin, PluginError, PluginInfo, PluginState};
use crate::{model::Model, plugin::Plugin};
use fly_ruler_utils::error::FrError;
use log::{trace, warn};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy)]
pub enum PluginType {
    System,
    Controller,
    Model,
}

#[derive(Debug)]
pub struct PluginManager {
    models: Vec<Model>,
    systems: Vec<Plugin>,
    controllers: Vec<Plugin>,
}

fn load_plugins<Pl: IsPlugin>(
    dir: PathBuf,
    loader: Box<dyn Fn(&Path) -> Result<Pl, PluginError>>,
) -> Vec<Pl> {
    let mut plugins = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            let sub_dir = entry.path();
            trace!("find plugin directory: {}", sub_dir.display());
            let plugin = loader(sub_dir);
            match plugin {
                Ok(p) => plugins.push(p),
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

        let models = load_plugins(model_path, Box::new(|dir| Model::new(dir)));
        let systems = load_plugins(system_path, Box::new(|dir| Plugin::new(dir)));
        let controllers = load_plugins(controller_path, Box::new(|dir| Plugin::new(dir)));

        Self {
            models,
            systems,
            controllers,
        }
    }

    pub fn get_infos(&self) -> Vec<Vec<&PluginInfo>> {
        let mi = self.models.iter().map(|p| p.info()).collect::<Vec<_>>();
        let si = self.models.iter().map(|p| p.info()).collect::<Vec<_>>();
        let ci = self.models.iter().map(|p| p.info()).collect::<Vec<_>>();
        vec![mi, si, ci]
    }

    pub fn install(
        &mut self,
        plugin_type: PluginType,
        index: usize,
        args: Vec<Box<dyn ToString>>,
    ) -> Result<(), FrError> {
        match plugin_type {
            PluginType::Model => match self.models.get_mut(index) {
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
        index: usize,
        args: Vec<Box<dyn ToString>>,
    ) -> Result<(), FrError> {
        match plugin_type {
            PluginType::Model => match self.models.get_mut(index) {
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

    pub fn get_model(&mut self, index: usize) -> Option<Arc<Mutex<Model>>> {
        match self.models.get_mut(index) {
            Some(model) => {
                if model.state() == &PluginState::Installed {
                    let model = self.models.remove(index);
                    Some(Arc::new(Mutex::new(model)))
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

    pub fn return_model(&mut self, model: Arc<Mutex<Model>>) {
        let model: Option<Mutex<Model>> = Arc::into_inner(model);
        match model {
            Some(model) => {
                let model = Mutex::into_inner(model);
                self.models.push(model);
            }
            None => warn!("return model failed"),
        }
    }
}
