use crate::manager::{AsPluginManager, ConfigManager, ModelManager};
use fly_ruler_core::core::{Core, CoreInitCfg, PlaneInitCfg};
use fly_ruler_plugin::{PluginInfo, PluginState};
use fly_ruler_utils::{
    error::FrError, input_channel, plane_model::Control, InputReceiver, InputSender, OutputReceiver,
};
use log::{error, info, warn};
use mlua::{LuaSerdeExt, UserData};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process,
};

pub struct System {
    config_root: PathBuf,
    model_root: PathBuf,
    config_manager: Option<ConfigManager>,
    model_manager: Option<ModelManager>,
    state_receivers: HashMap<usize, OutputReceiver>,
    control_receivers: HashMap<usize, InputReceiver>,
    error_handler: Box<dyn Fn(FrError)>,
    core: Option<Core>,
}

impl System {
    pub fn new(error_handler: Box<dyn Fn(FrError)>) -> Self {
        Self {
            config_root: PathBuf::from("config"),
            model_root: PathBuf::from("models"),
            config_manager: None,
            model_manager: None,
            state_receivers: HashMap::new(),
            control_receivers: HashMap::new(),
            core: None,
            error_handler,
        }
    }

    fn set_dir<P: AsRef<Path>>(&mut self, config_root_path: P, model_root_path: P) -> &mut Self {
        self.config_root = PathBuf::from(config_root_path.as_ref());
        self.model_root = PathBuf::from(model_root_path.as_ref());
        self
    }

    fn get_models(&self) -> HashMap<usize, (PluginInfo, PluginState)> {
        let infos = self.model_manager.as_ref().unwrap().infos();
        let states = self.model_manager.as_ref().unwrap().states();
        infos
            .iter()
            .zip(states.iter())
            .map(|(info, state)| (*info.0, (info.1.clone(), state.1.clone())))
            .collect()
    }

    fn enable_model(&mut self, index: usize, args: &[impl ToString]) {
        if let Some(manager) = &mut self.model_manager {
            let result = manager.enable(index, args);
            match result {
                Ok(()) => {}
                Err(e) => {
                    error!("{}", e);
                    (self.error_handler)(e);
                    self.err_stop()
                }
            }
        } else {
            warn!("Sys: model manager is not initialized");
        }
    }

    fn disable_model(&mut self, index: usize) {
        if let Some(manager) = &mut self.model_manager {
            let _ = manager.disable(index);
        } else {
            warn!("Sys: model manager is not initialized");
        }
    }

    fn get_model_state(&self, index: usize) -> Option<PluginState> {
        if let Some(manager) = &self.model_manager {
            manager.state(index)
        } else {
            warn!("Sys: model manager is not initialized");
            None
        }
    }

    async fn push_plane(&mut self, index: usize, init_cfg: PlaneInitCfg) {
        let model = self.model_manager.as_ref().unwrap().get_model(index);
        match model {
            Some(model) => match &mut self.core {
                Some(core) => {
                    let state_receiver = core.push_plane(model, init_cfg).await;
                    match state_receiver {
                        Ok(state_receiver) => {
                            self.state_receivers.insert(index, state_receiver);
                        }
                        Err(e) => {
                            error!("{}", e);
                            (self.error_handler)(e);
                            self.err_stop();
                        }
                    }
                }
                None => {
                    warn!("Sys: core is not initialized");
                }
            },
            None => {
                warn!("Sys: model is not available");
            }
        }
    }

    fn init(&mut self, init_cfg: CoreInitCfg) {
        self.config_manager = Some(ConfigManager::new(&self.config_root));
        self.model_manager = Some(ModelManager::new(&self.model_root));
        let core = Core::new(init_cfg);
        self.core = Some(core);
    }

    fn get_controller(&mut self, id: usize, init: Control) -> InputSender {
        let (tx, rx) = input_channel(init);
        self.control_receivers.insert(id, rx);
        tx
    }

    fn get_viewer(&mut self, id: usize) -> Option<OutputReceiver> {
        match self.state_receivers.get(&id) {
            Some(m) => {
                info!("set viewer for plane {id}");
                Some(m.clone())
            }
            None => {
                warn!("Sys: plane not found");
                None
            }
        }
    }

    async fn run(&mut self, is_block: bool) -> &mut Self {
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
        let p = self.model_manager.as_mut().unwrap();
        let _ = p.disable_all();
        error!("Sys: system didn't exit successfully");
        process::exit(1)
    }

    fn stop(&mut self) {
        let p = self.model_manager.as_mut().unwrap();
        let _ = p.disable_all();
        info!("system exited successfully");
    }
}

impl UserData for System {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("models", |lua, this| {
            let table = lua.create_table().unwrap();
            let methods = this.get_models();
            methods.iter().for_each(|(k, v)| {
                let t = lua.create_table().unwrap();
                t.set("info", lua.to_value(&v.0).unwrap()).unwrap();
                t.set("state", lua.to_value(&v.1).unwrap()).unwrap();
                table.set(*k + 1, t).unwrap();
            });
            Ok(table)
        });
    }
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("set_dir", |_, this, value: (String, String)| {
            this.set_dir(value.0, value.1);
            Ok(())
        });

        methods.add_method_mut("stop", |_, this, ()| {
            this.stop();
            Ok(())
        });

        methods.add_method_mut(
            "enable_model",
            |_, this, (index, args): (usize, Vec<String>)| Ok(this.enable_model(index - 1, &args)),
        );

        methods.add_method_mut("disable_model", |_, this, index: usize| {
            Ok(this.disable_model(index - 1))
        });

        methods.add_method_mut("get_model_state", |lua, this, index: usize| {
            Ok(lua.to_value(&this.get_model_state(index - 1)))
        });

        methods.add_method_mut(
            "get_controller",
            |lua, this, (index, init): (usize, mlua::Table)| {
                Ok(this.get_controller(index - 1, lua.from_value(mlua::Value::Table(init))?))
            },
        );

        methods.add_method_mut("get_viewer", |_lua, this, index: usize| {
            Ok(this.get_viewer(index - 1))
        });

        methods.add_async_method_mut(
            "push_plane",
            |lua, this, (index, init_cfg): (usize, mlua::Value)| async move {
                let init_cfg: Result<PlaneInitCfg, mlua::prelude::LuaError> =
                    lua.from_value(init_cfg);
                match init_cfg {
                    Ok(cfg) => Ok(this.push_plane(index - 1, cfg).await),
                    Err(e) => {
                        error!("{}", e);
                        return Err(e);
                    }
                }
            },
        );

        methods.add_method_mut("init", |lua, this, init_cfg: mlua::Value| {
            let init_cfg: Result<CoreInitCfg, mlua::prelude::LuaError> = lua.from_value(init_cfg);
            match init_cfg {
                Ok(cfg) => Ok(this.init(cfg)),
                Err(e) => {
                    error!("{}", e);
                    return Err(e);
                }
            }
        });

        methods.add_async_method_mut("run", |_, this, is_block: bool| async move {
            this.run(is_block).await;
            Ok(())
        })
    }
}
