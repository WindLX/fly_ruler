use crate::config::ConfigManager;
use fly_ruler_core::core::Core;
use fly_ruler_plugin::{PluginInfo, PluginManager, PluginType};
use fly_ruler_utils::{
    error::FrError,
    plane_model::{Control, CoreOutput},
    StateReceiver,
};
use log::{error, info, warn};
use std::{collections::HashMap, path::PathBuf, process, time::Duration};

pub struct System {
    config_root: PathBuf,
    plugin_root: PathBuf,
    config_manager: Option<ConfigManager>,
    plugin_manager: Option<PluginManager>,
    state_receivers: HashMap<usize, StateReceiver>,
    control_tasks: Vec<tokio::task::JoinHandle<Result<(), FrError>>>,
    error_handler: Box<dyn Fn(FrError)>,
    core: Core,
}

impl System {
    pub fn builder(error_handler: Box<dyn Fn(FrError)>) -> Self {
        Self {
            config_root: PathBuf::from("config"),
            plugin_root: PathBuf::from("plugins"),
            config_manager: None,
            plugin_manager: None,
            state_receivers: HashMap::new(),
            control_tasks: Vec::new(),
            core: Core::new(),
            error_handler,
        }
    }

    pub fn set_dir(&mut self, config_root_path: PathBuf, plugin_root_path: PathBuf) -> &mut Self {
        self.config_root = config_root_path;
        self.plugin_root = plugin_root_path;
        self
    }

    pub async fn init<'h, 's: 'h>(
        &'s mut self,
        model_handler: Option<
            impl FnOnce(&HashMap<usize, PluginInfo>) -> &'h [(usize, &'h [&str])],
        >,
        control_handler: Option<
            impl FnOnce(&HashMap<usize, PluginInfo>) -> &'h [(usize, &'h [&str])],
        >,
        system_handler: Option<
            impl FnOnce(&HashMap<usize, PluginInfo>) -> &'h [(usize, &'h [&str])],
        >,
    ) -> &'s mut Self {
        self.config_manager = Some(ConfigManager::new(&self.config_root));
        self.plugin_manager = Some(PluginManager::new(&self.plugin_root));

        let core_init_cfg = self.config_manager.as_ref().unwrap().load_core_init();
        if let Err(e) = core_init_cfg {
            error!("{}", e);
            (self.error_handler)(e);
            self.err_stop().await;
        }
        let core_init_cfg = core_init_cfg.unwrap();

        let infos = self.plugin_manager.as_ref().unwrap().get_infos().await;

        if let Some(h) = control_handler {
            let control_plugins = h(&infos[0]);
        }

        if let Some(h) = system_handler {
            let system_plugins = h(&infos[0]);
        }

        if let Some(h) = model_handler {
            let model_plugins = h(&infos[0]);
            for i in model_plugins {
                let p = self.plugin_manager.as_mut().unwrap();
                let r = p.install(PluginType::Model, &i.0, i.1).await;
                if let Err(e) = r {
                    error!("{}", e);
                    (self.error_handler)(e);
                    self.err_stop().await;
                }

                let model = p.get_model(&i.0).await;
                if let Some(m) = model {
                    let state_receiver = self.core.push_plane(i.0, m, core_init_cfg).await;
                    match state_receiver {
                        Ok(state_receiver) => {
                            self.state_receivers.insert(i.0, state_receiver);
                        }
                        Err(e) => {
                            error!("{}", e);
                            (self.error_handler)(e);
                            self.err_stop().await;
                        }
                    }
                }
            }
        }

        self
    }

    pub async fn set_controller<'h, 's: 'h>(
        &'s mut self,
        handler: impl FnOnce(
            &Vec<(usize, PluginInfo)>,
            &HashMap<usize, PluginInfo>,
        ) -> (usize, &'h [usize]),
    ) {
    }

    pub async fn run(
        &mut self,
        step_handler: impl Fn(Duration, &HashMap<usize, CoreOutput>) -> bool,
    ) -> &mut Self {
        self.core.start().await;
        loop {
            let output = self.core.step(&[]).await;
            match output {
                Err(e) => {
                    error!("{}", e);
                    (self.error_handler)(e);
                    self.err_stop().await;
                }
                Ok(output) => match output {
                    Err(e) => {
                        warn!("Core: {e}");
                    }
                    Ok(output) => {
                        if step_handler(self.core.get_time().await, &output) {
                            break;
                        }
                    }
                },
            }
        }
        self
    }

    async fn err_stop(&mut self) -> ! {
        let p = self.plugin_manager.as_mut().unwrap();
        let _ = p.uninstall_all().await;
        process::exit(1)
    }

    pub async fn stop(&mut self) -> ! {
        let p = self.plugin_manager.as_mut().unwrap();
        let _ = p.uninstall_all().await;
        info!("system exit successfully");
        process::exit(0)
    }
}
