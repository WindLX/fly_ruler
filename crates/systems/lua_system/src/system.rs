use crate::manager::{AsPluginManager, ModelManager};
use fly_ruler_core::core::{Core, CoreInitCfg, PlaneInitCfg};
use fly_ruler_plugin::{PluginInfo, PluginState};
use fly_ruler_utils::{input_channel, InputSender, OutputReceiver};
use log::{error, info, trace, warn};
use lua_runtime::{InputSenderWrapper, OutputReceiverWrapper, UuidWrapper};
use mlua::{IntoLua, LuaSerdeExt, UserData, UserDataRef};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process,
    sync::{Arc, Mutex},
    time::Duration,
};
use uuid::Uuid;

pub struct System {
    model_root: PathBuf,
    model_manager: Option<ModelManager>,
    core: Option<Core>,
}

#[derive(Clone)]
pub struct SystemWrapper(pub Arc<Mutex<System>>);

impl SystemWrapper {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(System::new())))
    }
}

impl System {
    pub fn new() -> Self {
        Self {
            model_root: PathBuf::from("models"),
            model_manager: None,
            core: None,
        }
    }

    pub fn set_dir<P: AsRef<Path>>(&mut self, model_root_path: P) {
        self.model_root = PathBuf::from(model_root_path.as_ref());
    }

    pub fn get_models(&self) -> HashMap<Uuid, (PluginInfo, PluginState)> {
        let infos = self.model_manager.as_ref().unwrap().infos();
        let states = self.model_manager.as_ref().unwrap().states();
        infos
            .iter()
            .zip(states.iter())
            .map(|(info, state)| (*info.0, (info.1.clone(), state.1.clone())))
            .collect()
    }

    pub fn enable_model(&mut self, model_id: Uuid, args: &[impl ToString]) {
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

    pub fn disable_model(&mut self, model_id: Uuid) {
        if let Some(manager) = &mut self.model_manager {
            let _ = manager.disable(model_id);
        } else {
            warn!("Sys: model manager is not initialized");
        }
    }

    pub fn get_model_state(&self, model_id: Uuid) -> Option<PluginState> {
        if let Some(manager) = &self.model_manager {
            manager.state(model_id)
        } else {
            warn!("Sys: model manager is not initialized");
            None
        }
    }

    pub async fn push_plane(
        &mut self,
        model_id: Uuid,
        init_cfg: PlaneInitCfg,
    ) -> Option<(Uuid, OutputReceiver)> {
        let model = self.model_manager.as_ref().unwrap().get_model(model_id);
        match model {
            Some(model) => match &mut self.core {
                Some(core) => {
                    let state_receiver = core.push_plane(model, init_cfg).await;
                    match state_receiver {
                        Ok(state_receiver) => Some(state_receiver),
                        Err(e) => {
                            error!("{}", e);
                            self.err_stop();
                        }
                    }
                }
                None => {
                    warn!("Sys: core is not initialized");
                    None
                }
            },
            None => {
                warn!("Sys: model is not available");
                None
            }
        }
    }

    pub fn init(&mut self, init_cfg: CoreInitCfg) {
        self.model_manager = Some(ModelManager::new(&self.model_root));
        let core = Core::new(init_cfg);
        self.core = Some(core);
    }

    pub fn set_controller(&mut self, plane_id: Uuid, buffer: usize) -> Option<InputSender> {
        let (tx, rx) = input_channel(buffer);
        match &mut self.core {
            Some(core) => {
                core.set_controller(plane_id, rx);
                Some(tx)
            }
            None => {
                warn!("Sys: core is not initialized");
                None
            }
        }
    }

    pub fn subscribe_plane(&self, plane_id: Uuid) -> Option<OutputReceiver> {
        match &self.core {
            Some(core) => core.subscribe_plane(plane_id),
            None => {
                warn!("Sys: core is not initialized");
                None
            }
        }
    }

    pub async fn remove_plane(&mut self, plane_id: Uuid) {
        match &mut self.core {
            Some(core) => core.remove_plane(plane_id).await,
            None => {
                warn!("Sys: core is not initialized");
            }
        }
    }

    pub async fn step(&mut self) {
        match &mut self.core {
            Some(core) => {
                let output = core.step().await;
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
                warn!("Sys: core is not initialized");
            }
        }
    }

    pub async fn pause(&self) {
        match &self.core {
            Some(core) => {
                core.pause().await;
                info!("core has been paused");
            }
            None => {
                warn!("Sys: core is not initialized");
            }
        }
    }

    pub async fn resume(&self) {
        match &self.core {
            Some(core) => {
                core.resume().await;
                info!("core has been resumed");
            }
            None => {
                warn!("Sys: core is not initialized");
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

    pub fn contains_plane(&self, plane_id: Uuid) -> Option<bool> {
        match &self.core {
            Some(core) => Some(core.contains_plane(plane_id)),
            None => {
                warn!("Sys: core is not initialized");
                None
            }
        }
    }

    pub fn planes(&self) -> Option<Vec<Uuid>> {
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

impl UserData for SystemWrapper {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("models", |lua, this| {
            let table = lua.create_table().unwrap();
            let methods = this.0.lock().unwrap().get_models();
            methods.iter().for_each(|(k, v)| {
                let t = lua.create_table().unwrap();
                t.set("info", lua.to_value(&v.0).unwrap()).unwrap();
                t.set("state", lua.to_value(&v.1).unwrap()).unwrap();
                table.set(UuidWrapper::from(*k), t).unwrap();
            });
            Ok(table)
        });

        fields.add_field_method_get("planes", |_lua, this| {
            Ok(match this.0.lock().unwrap().planes() {
                Some(planes) => Some(
                    planes
                        .into_iter()
                        .map(|p| UuidWrapper::from(p))
                        .collect::<Vec<_>>(),
                ),
                None => None,
            })
        })
    }
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        // system
        methods.add_method_mut("set_dir", |_, this, value: String| {
            this.0.lock().unwrap().set_dir(value);
            Ok(())
        });

        methods.add_method_mut("init", |lua, this, init_cfg: mlua::Value| {
            let init_cfg: Result<CoreInitCfg, mlua::prelude::LuaError> = lua.from_value(init_cfg);
            match init_cfg {
                Ok(cfg) => Ok(this.0.lock().unwrap().init(cfg)),
                Err(e) => {
                    error!("{}", e);
                    Err(e)
                }
            }
        });

        methods.add_async_method_mut("step", |_, this, ()| async move {
            this.0.lock().unwrap().step().await;
            Ok(())
        });

        methods.add_method_mut("stop", |_, this, ()| {
            this.0.lock().unwrap().stop();
            Ok(())
        });

        // model
        methods.add_method_mut(
            "enable_model",
            |_, this, (index, args): (UserDataRef<'lua, UuidWrapper>, Vec<String>)| {
                Ok(this.0.lock().unwrap().enable_model(index.inner(), &args))
            },
        );

        methods.add_method_mut(
            "disable_model",
            |_, this, index: UserDataRef<'lua, UuidWrapper>| {
                Ok(this.0.lock().unwrap().disable_model(index.inner()))
            },
        );

        methods.add_method_mut(
            "get_model_state",
            |lua, this, index: UserDataRef<'lua, UuidWrapper>| {
                Ok(lua.to_value(&this.0.lock().unwrap().get_model_state(index.inner())))
            },
        );

        // input and output
        methods.add_method_mut(
            "set_controller",
            |_lua, this, (index, buffer): (UserDataRef<'lua, UuidWrapper>, usize)| {
                let controller = this.0.lock().unwrap().set_controller(index.inner(), buffer);
                let c = match controller {
                    Some(c) => Some(InputSenderWrapper::from(c)),
                    None => None,
                };
                Ok(c)
            },
        );

        methods.add_method(
            "subscribe_plane",
            |_lua, this, index: UserDataRef<'lua, UuidWrapper>| {
                let viewer = this.0.lock().unwrap().subscribe_plane(index.inner());
                let v = match viewer {
                    Some(viewer) => Some(OutputReceiverWrapper::from(viewer)),
                    None => None,
                };
                Ok(v)
            },
        );

        // plane
        methods.add_async_method_mut(
            "push_plane",
            |lua, this, (index, init_cfg): (UserDataRef<'lua, UuidWrapper>, mlua::Value)| async move {
                let init_cfg: Result<PlaneInitCfg, mlua::prelude::LuaError> =
                    lua.from_value(init_cfg);
                match init_cfg {
                    Ok(cfg) => Ok(match this
                        .0.lock().unwrap().push_plane(index.inner(), cfg)
                        .await {
                            Some(o) => {
                                let t = lua.create_table()?;
                                t.push(UuidWrapper::from(o.0))?;
                                t.push(OutputReceiverWrapper::from(o.1))?;
                            t.into_lua(lua)}
                            None => Ok(mlua::Value::Nil)
                        }),
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
        );

        methods.add_async_method_mut(
            "remove_plane",
            |_lua, this, id: UserDataRef<'lua, UuidWrapper>| async move {
                this.0.lock().unwrap().remove_plane(id.inner()).await;
                Ok(())
            },
        );

        methods.add_method(
            "contain_plane",
            |_, this, index: UserDataRef<'lua, UuidWrapper>| {
                Ok(this.0.lock().unwrap().contains_plane(index.inner()))
            },
        );

        // time
        methods.add_async_method("get_time", |lua, this, ()| async move {
            let time = this.0.lock().unwrap().get_time().await;
            match time {
                Some(t) => Ok(t.as_millis().into_lua(lua)),
                None => Ok(mlua::Nil.into_lua(lua)),
            }
        });

        methods.add_async_method("pause", |_lua, this, ()| async move {
            this.0.lock().unwrap().pause().await;
            Ok(())
        });

        methods.add_async_method("resume", |_lua, this, ()| async move {
            this.0.lock().unwrap().pause().await;
            Ok(())
        });

        methods.add_method("clone", |_lua, this, ()| Ok(this.clone()));
    }
}
