use crate::manager::{AsPluginManager, ModelManager};
use fly_ruler_core::core::{Core, CoreInitCfg, PlaneInitCfg};
use fly_ruler_plugin::{PluginInfo, PluginState};
use fly_ruler_utils::{
    input_channel, plane_model::Control, InputReceiver, InputSender, OutputReceiver,
};
use log::{error, info, trace, warn};
use lua_runtime::{InputSenderWrapper, OutputReceiverWrapper};
use mlua::{IntoLua, LuaSerdeExt, UserData};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process,
    time::Duration,
};

pub struct System {
    model_root: PathBuf,
    model_manager: Option<ModelManager>,
    state_receivers: HashMap<usize, OutputReceiver>,
    control_receivers: HashMap<usize, InputReceiver>,
    core: Option<Core>,
}

impl System {
    pub fn new() -> Self {
        Self {
            model_root: PathBuf::from("models"),
            model_manager: None,
            state_receivers: HashMap::new(),
            control_receivers: HashMap::new(),
            core: None,
        }
    }

    pub fn set_dir<P: AsRef<Path>>(&mut self, model_root_path: P) {
        self.model_root = PathBuf::from(model_root_path.as_ref());
    }

    pub fn get_models(&self) -> HashMap<usize, (PluginInfo, PluginState)> {
        let infos = self.model_manager.as_ref().unwrap().infos();
        let states = self.model_manager.as_ref().unwrap().states();
        infos
            .iter()
            .zip(states.iter())
            .map(|(info, state)| (*info.0, (info.1.clone(), state.1.clone())))
            .collect()
    }

    pub fn enable_model(&mut self, model_id: usize, args: &[impl ToString]) {
        if let Some(manager) = &mut self.model_manager {
            let result = manager.enable(model_id, args);
            match result {
                Ok(()) => {}
                Err(e) => {
                    error!("{}", e);
                    self.err_stop()
                }
            }
        } else {
            warn!("Sys: model manager is not initialized");
        }
    }

    pub fn disable_model(&mut self, model_id: usize) {
        if let Some(manager) = &mut self.model_manager {
            let _ = manager.disable(model_id);
        } else {
            warn!("Sys: model manager is not initialized");
        }
    }

    pub fn get_model_state(&self, model_id: usize) -> Option<PluginState> {
        if let Some(manager) = &self.model_manager {
            manager.state(model_id)
        } else {
            warn!("Sys: model manager is not initialized");
            None
        }
    }

    pub async fn push_plane(&mut self, model_id: usize, init_cfg: PlaneInitCfg) {
        let model = self.model_manager.as_ref().unwrap().get_model(model_id);
        match model {
            Some(model) => match &mut self.core {
                Some(core) => {
                    let state_receiver = core.push_plane(model, init_cfg).await;
                    match state_receiver {
                        Ok(state_receiver) => {
                            self.state_receivers
                                .insert(core.plane_count() - 1, state_receiver);
                        }
                        Err(e) => {
                            error!("{}", e);
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

    pub fn init(&mut self, init_cfg: CoreInitCfg) {
        self.model_manager = Some(ModelManager::new(&self.model_root));
        let core = Core::new(init_cfg);
        self.core = Some(core);
    }

    pub fn set_controller(&mut self, plane_id: usize, init: Control) -> InputSender {
        let (tx, rx) = input_channel(init);
        self.control_receivers.insert(plane_id, rx);
        info!("set controller for plane {plane_id}");
        tx
    }

    pub fn get_viewer(&mut self, plane_id: usize) -> Option<OutputReceiver> {
        match self.state_receivers.get(&plane_id) {
            Some(m) => {
                info!("set viewer for plane {plane_id}");
                Some(m.clone())
            }
            None => {
                warn!("Sys: plane not found");
                None
            }
        }
    }

    pub async fn run(&mut self, is_block: bool) {
        match &mut self.core {
            Some(core) => {
                core.start().await;
                let output = core.run(is_block, &self.control_receivers).await;
                match output {
                    Err(e) => {
                        error!("{}", e);
                        self.err_stop();
                    }
                    Ok(_) => {
                        info!("system running task fininshed");
                    }
                }
            }
            None => {
                error!("Sys: core is not initialized");
                self.err_stop()
            }
        }
    }

    pub async fn start(&mut self) {
        match &mut self.core {
            Some(core) => {
                core.start().await;
            }
            None => {
                error!("Sys: core is not initialized");
                self.err_stop()
            }
        }
    }

    pub async fn step(&mut self, is_block: bool) {
        match &mut self.core {
            Some(core) => {
                let output = core.step(is_block, &self.control_receivers).await;
                match output {
                    Err(e) => {
                        error!("{}", e);
                        self.err_stop();
                    }
                    Ok(_) => {
                        trace!("system step task fininshed");
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

    pub fn stop(&mut self) {
        let p = self.model_manager.as_mut().unwrap();
        let _ = p.disable_all();
        info!("system exited successfully");
    }

    pub fn contains_plane(&self, plane_id: usize) -> Option<bool> {
        match &self.core {
            Some(core) => Some(core.contains_plane(plane_id)),
            None => {
                warn!("Sys: core is not initialized");
                None
            }
        }
    }

    pub fn planes(&self) -> Option<Vec<usize>> {
        match &self.core {
            Some(core) => Some(core.planes()),
            None => {
                warn!("Sys: core is not initialized");
                None
            }
        }
    }

    pub async fn get_time(&self) -> Option<Duration> {
        match &self.core {
            Some(core) => Some(core.get_time().await),
            None => {
                warn!("Sys: core is not initialized");
                None
            }
        }
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

        fields.add_field_method_get("planes", |_lua, this| Ok(this.planes()));
    }
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        // system
        methods.add_method_mut("set_dir", |_, this, value: String| {
            this.set_dir(value);
            Ok(())
        });

        methods.add_method_mut("init", |lua, this, init_cfg: mlua::Value| {
            let init_cfg: Result<CoreInitCfg, mlua::prelude::LuaError> = lua.from_value(init_cfg);
            match init_cfg {
                Ok(cfg) => Ok(this.init(cfg)),
                Err(e) => {
                    error!("{}", e);
                    Err(e)
                }
            }
        });

        methods.add_async_method_mut("run", |_, this, is_block: bool| async move {
            this.run(is_block).await;
            Ok(())
        });

        methods.add_async_method_mut("step", |_, this, is_block: bool| async move {
            this.step(is_block).await;
            Ok(())
        });

        methods.add_async_method_mut("start", |_, this, ()| async move {
            this.start().await;
            Ok(())
        });

        methods.add_method_mut("stop", |_, this, ()| {
            this.stop();
            Ok(())
        });

        // model
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

        // input and output
        methods.add_method_mut(
            "set_controller",
            |lua, this, (index, init): (usize, mlua::Table)| {
                Ok(InputSenderWrapper::from(this.set_controller(
                    index - 1,
                    lua.from_value(mlua::Value::Table(init))?,
                )))
            },
        );

        methods.add_method_mut("get_viewer", |_lua, this, index: usize| {
            let viewer = this.get_viewer(index - 1);
            let v = match viewer {
                Some(viewer) => Some(OutputReceiverWrapper::from(viewer)),
                None => None,
            };
            Ok(v)
        });

        // plane
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

        methods.add_method("contain_plane", |_, this, index: usize| {
            Ok(this.contains_plane(index - 1))
        });

        // time
        methods.add_async_method("get_time", |lua, this, ()| async move {
            let time = this.get_time().await;
            match time {
                Some(t) => Ok(t.as_millis().into_lua(lua)),
                None => Ok(mlua::Nil.into_lua(lua)),
            }
        });
    }
}
