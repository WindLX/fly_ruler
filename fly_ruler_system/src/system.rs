use crate::config::ConfigManager;
use fly_ruler_core::core::Core;
use fly_ruler_plugin::{PluginInfo, PluginManager, PluginType};
use fly_ruler_utils::{error::FrError, CommandReceiver, OutputReceiver};
use log::{error, info};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process,
};

pub struct System {
    config_root: PathBuf,
    plugin_root: PathBuf,
    config_manager: Option<ConfigManager>,
    plugin_manager: Option<PluginManager>,
    state_receivers: HashMap<usize, OutputReceiver>,
    control_receivers: HashMap<usize, CommandReceiver>,
    error_handler: Box<dyn Fn(FrError)>,
    core: Option<Core>,
}

impl System {
    pub fn builder(error_handler: Box<dyn Fn(FrError)>) -> Self {
        Self {
            config_root: PathBuf::from("config"),
            plugin_root: PathBuf::from("plugins"),
            config_manager: None,
            plugin_manager: None,
            state_receivers: HashMap::new(),
            control_receivers: HashMap::new(),
            core: None,
            error_handler,
        }
    }

    pub fn set_dir<P: AsRef<Path>>(
        &mut self,
        config_root_path: P,
        plugin_root_path: P,
    ) -> &mut Self {
        self.config_root = PathBuf::from(config_root_path.as_ref());
        self.plugin_root = PathBuf::from(plugin_root_path.as_ref());
        self
    }

    pub async fn init<'s>(
        &mut self,
        model_handler: Option<
            impl FnOnce(&HashMap<usize, PluginInfo>) -> Vec<(usize, Vec<String>)>,
        >,
    ) -> &mut Self {
        self.config_manager = Some(ConfigManager::new(&self.config_root));
        self.plugin_manager = Some(PluginManager::new(&self.plugin_root));

        let core_init_cfg = self.config_manager.as_ref().unwrap().load_core_init();
        if let Err(e) = core_init_cfg {
            error!("{}", e);
            (self.error_handler)(e);
            self.err_stop();
        }
        let core_init_cfg = core_init_cfg.unwrap();
        let mut core = Core::new(core_init_cfg);

        let infos = self.plugin_manager.as_ref().unwrap().get_infos();

        if let Some(h) = model_handler {
            let model_plugins = h(&infos[0]);
            for i in model_plugins {
                let p = self.plugin_manager.as_mut().unwrap();
                let r = p.install(PluginType::Model, &i.0, &i.1);
                if let Err(e) = r {
                    error!("{}", e);
                    (self.error_handler)(e);
                    self.err_stop();
                }

                let model = p.get_model(&i.0);
                if let Some(m) = model {
                    let state_receiver = core.push_plane(i.0, m).await;
                    match state_receiver {
                        Ok(state_receiver) => {
                            self.state_receivers.insert(i.0, state_receiver);
                        }
                        Err(e) => {
                            error!("{}", e);
                            (self.error_handler)(e);
                            self.err_stop();
                        }
                    }
                }
            }
        }
        self.core = Some(core);

        self
    }

    pub async fn set_controller(
        &mut self,
        handler: impl FnOnce(&Vec<(usize, PluginInfo)>) -> HashMap<usize, CommandReceiver>,
    ) -> &mut Self {
        match &mut self.core {
            Some(core) => {
                let ids = core
                    .get_ids()
                    .iter()
                    .map(|(p, k)| match &self.plugin_manager {
                        Some(m) => {
                            let h = (*p, m.get_infos()[0].get(k).unwrap().clone());
                            info!("set controller for plane {}", p);
                            h
                        }
                        None => {
                            error!("Sys: plugin manager is not initialized");
                            self.err_stop()
                        }
                    })
                    .collect::<Vec<(usize, PluginInfo)>>();
                self.control_receivers = handler(&ids);
                self
            }
            None => {
                error!("Sys: core is not initialized");
                self.err_stop()
            }
        }
    }

    pub async fn set_viewer(
        &mut self,
        id: usize,
        handler: impl FnOnce(OutputReceiver),
    ) -> &mut Self {
        match self.state_receivers.get(&id) {
            Some(m) => {
                handler(m.clone());
                info!("set viewer for plane {id}");
                self
            }
            None => {
                error!("Sys: plane not found");
                self.err_stop()
            }
        }
    }

    pub async fn run(&mut self, is_block: bool) -> &mut Self {
        match &mut self.core {
            Some(core) => {
                core.start().await;
                let output = core.run(is_block, &self.control_receivers).await;
                match output {
                    Err(e) => {
                        error!("{}", e);
                        (self.error_handler)(e);
                        self.err_stop();
                    }
                    Ok(_) => {
                        info!("system running task fininshed");
                        self
                    }
                }
            }
            None => {
                error!("Sys: core is not initialized");
                self.err_stop()
            }
        }
    }

    fn err_stop(&mut self) -> ! {
        let p = self.plugin_manager.as_mut().unwrap();
        let _ = p.uninstall_all();
        error!("Sys: system didn't exit successfully");
        process::exit(1)
    }

    pub fn stop(&mut self) {
        let p = self.plugin_manager.as_mut().unwrap();
        let _ = p.uninstall_all();
        info!("system exited successfully");
    }
}
