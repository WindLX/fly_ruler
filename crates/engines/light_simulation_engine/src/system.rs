use crate::manager::{AsPluginManager, ModelManager};
use fly_ruler_core::core::{Core, CoreInitCfg, PlaneInitCfg};
use fly_ruler_plugin::{PluginInfo, PluginState};
use fly_ruler_utils::{error::FrError, input_channel, InputSender, OutputReceiver};
use lua_runtime::prelude::*;
use lua_runtime::{InputSenderWrapper, OutputReceiverWrapper, UuidWrapper};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Mutex,
    time::Duration,
};
use thiserror::Error;
use tracing::{event, instrument, Level};
use uuid::Uuid;

pub struct System {
    model_root: PathBuf,
    model_manager: Option<ModelManager>,
    core: Option<Core>,
}

#[derive(Clone)]
pub struct SystemWrapper(pub Rc<Mutex<System>>);

impl SystemWrapper {
    pub fn new() -> Self {
        Self(Rc::new(Mutex::new(System::new())))
    }
}
impl System {
    #[instrument(level = Level::INFO)]
    pub fn new() -> Self {
        Self {
            model_root: PathBuf::from("models"),
            model_manager: None,
            core: None,
        }
    }

    #[instrument(skip_all,level = Level::INFO)]
    pub fn set_dir<P: AsRef<Path>>(&mut self, model_root_path: P) {
        self.model_root = PathBuf::from(model_root_path.as_ref());
    }

    #[instrument(skip_all, level = Level::INFO, err, ret)]
    pub fn get_models(&self) -> Result<HashMap<Uuid, (PluginInfo, PluginState)>, SysError> {
        if let Some(manager) = &self.model_manager {
            let infos = manager.infos();
            let states = manager.states();
            let r = infos
                .iter()
                .zip(states.iter())
                .map(|(info, state)| (*info.0, (info.1.clone(), state.1.clone())))
                .collect();
            Ok(r)
        } else {
            Err(SysError::ManagerNotInit)
        }
    }

    #[instrument(skip(self, args), level = Level::INFO, err)]
    pub fn enable_model(&mut self, model_id: Uuid, args: &[impl ToString]) -> Result<(), SysError> {
        if let Some(manager) = &mut self.model_manager {
            manager.enable(model_id, args)?;
            Ok(())
        } else {
            Err(SysError::ManagerNotInit)
        }
    }

    #[instrument(skip(self), level = Level::INFO, err)]
    pub fn disable_model(&mut self, model_id: Uuid) -> Result<(), SysError> {
        if let Some(manager) = &mut self.model_manager {
            manager.disable(model_id)?;
            Ok(())
        } else {
            Err(SysError::ManagerNotInit)
        }
    }

    #[instrument(skip(self), level = Level::INFO, err, ret)]
    pub fn get_model_state(&self, model_id: Uuid) -> Result<Option<PluginState>, SysError> {
        if let Some(manager) = &self.model_manager {
            let r = manager.state(model_id);
            Ok(r)
        } else {
            Err(SysError::ManagerNotInit)
        }
    }

    #[instrument(skip_all, level = Level::INFO)]
    pub fn init(&mut self, init_cfg: CoreInitCfg) {
        self.model_manager = Some(ModelManager::new(&self.model_root));
        let core = Core::new(init_cfg);
        self.core = Some(core);
    }

    #[instrument(skip(self, init_cfg), level = Level::INFO, err)]
    pub async fn push_plane(
        &mut self,
        model_id: Uuid,
        plane_id: Option<Uuid>,
        init_cfg: PlaneInitCfg,
    ) -> Result<(Uuid, OutputReceiver), SysError> {
        let model = if let Some(manager) = &mut self.model_manager {
            manager.get_model(model_id)
        } else {
            return Err(SysError::ManagerNotInit);
        };
        match model {
            Some(model) => match &mut self.core {
                Some(core) => Ok(core.push_plane(model, plane_id, init_cfg).await?),
                None => Err(SysError::CoreNotInit),
            },
            None => Err(SysError::ModelNotAvailable),
        }
    }

    #[instrument(skip(self), err)]
    pub async fn set_controller(
        &mut self,
        plane_id: Uuid,
        buffer: usize,
    ) -> Result<InputSender, SysError> {
        let (tx, rx) = input_channel(buffer);
        match &mut self.core {
            Some(core) => {
                core.set_controller(plane_id, rx).await;
                Ok(tx)
            }
            None => Err(SysError::CoreNotInit),
        }
    }

    #[instrument(skip(self), level = Level::INFO, err)]
    pub fn subscribe_plane(&self, plane_id: Uuid) -> Result<Option<OutputReceiver>, SysError> {
        match &self.core {
            Some(core) => Ok(core.subscribe_plane(plane_id)),
            None => Err(SysError::CoreNotInit),
        }
    }

    #[instrument(skip(self), level = Level::INFO)]
    pub async fn remove_plane(&mut self, plane_id: Uuid) {
        match &mut self.core {
            Some(core) => core.remove_plane(plane_id).await,
            None => {}
        }
    }

    pub fn plane_count(&mut self) -> Result<usize, SysError> {
        match &mut self.core {
            Some(core) => Ok(core.plane_count()),
            None => Err(SysError::CoreNotInit),
        }
    }

    #[instrument(skip(self), level = Level::INFO, err)]
    pub async fn step(&mut self, is_block: bool) -> Result<Result<(), FrError>, SysError> {
        match &mut self.core {
            Some(core) => {
                let r = core.step(is_block).await;
                Ok(r)
            }
            None => Err(SysError::CoreNotInit),
        }
    }

    #[instrument(skip(self), level = Level::INFO, err)]
    pub async fn pause(&self) -> Result<(), SysError> {
        match &self.core {
            Some(core) => {
                core.pause().await;
                event!(Level::INFO, "core has been paused");
                Ok(())
            }
            None => Err(SysError::CoreNotInit),
        }
    }

    #[instrument(skip(self), level = Level::INFO, err)]
    pub async fn resume(&self) -> Result<(), SysError> {
        match &self.core {
            Some(core) => {
                core.resume().await;
                event!(Level::INFO, "core has been resumed");
                Ok(())
            }
            None => Err(SysError::CoreNotInit),
        }
    }

    #[instrument(skip(self), level = Level::ERROR)]
    pub fn err_stop(&mut self) {
        let p = self.model_manager.as_mut().unwrap();
        let _ = p.disable_all();
    }

    #[instrument(skip(self), level = Level::INFO)]
    pub fn stop(&mut self) {
        let p = self.model_manager.as_mut().unwrap();
        let _ = p.disable_all();
        event!(Level::INFO, "system exited successfully");
    }

    #[instrument(skip(self), level = Level::INFO, err)]
    pub fn contains_plane(&self, plane_id: Uuid) -> Result<bool, SysError> {
        match &self.core {
            Some(core) => Ok(core.contains_plane(plane_id)),
            None => Err(SysError::CoreNotInit),
        }
    }

    #[instrument(skip(self), level = Level::INFO, err, ret)]
    pub fn planes(&self) -> Result<Vec<Uuid>, SysError> {
        match &self.core {
            Some(core) => Ok(core.planes()),
            None => Err(SysError::CoreNotInit),
        }
    }

    #[instrument(skip(self), level = Level::INFO, err, ret)]
    pub async fn get_time(&self) -> Result<Duration, SysError> {
        match &self.core {
            Some(core) => Ok(core.get_time().await),
            None => Err(SysError::CoreNotInit),
        }
    }
}

#[derive(Debug, Error)]
pub enum SysError {
    #[error("Sys: model manager is not initialized")]
    ManagerNotInit,
    #[error("Sys: core is not initialized")]
    CoreNotInit,
    #[error("{0}")]
    Fr(#[from] FrError),
    #[error("Model not available")]
    ModelNotAvailable,
}

impl LuaUserData for SystemWrapper {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("models", |lua, this| {
            let table = lua.create_table()?;
            let methods: HashMap<Uuid, (PluginInfo, PluginState)> = this
                .0
                .lock()
                .unwrap()
                .get_models()
                .map_err(LuaError::external)?;
            for (k, v) in methods.iter() {
                let t = lua.create_table()?;
                t.set("info", lua.to_value(&v.0)?)?;
                t.set("state", lua.to_value(&v.1)?)?;
                table.set(UuidWrapper::from(*k), t)?;
            }
            Ok(table)
        });

        fields.add_field_method_get("planes", |lua, this| {
            let r = this
                .0
                .lock()
                .unwrap()
                .planes()
                .map_err(LuaError::external)?;
            let table = lua.create_table()?;
            for p in r.into_iter() {
                table.push(UuidWrapper::from(p))?;
            }
            Ok(table)
        })
    }
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        // system
        methods.add_method_mut("set_dir", |_, this, value: String| {
            this.0.lock().unwrap().set_dir(value);
            Ok(())
        });

        methods.add_method_mut("init", |lua, this, init_cfg: LuaValue| {
            let init_cfg: CoreInitCfg = lua.from_value(init_cfg)?;
            Ok(this.0.lock().unwrap().init(init_cfg))
        });

        methods.add_async_method_mut("step", |lua, this, is_block: bool| async move {
            let r = this
                .0
                .lock()
                .unwrap()
                .step(is_block)
                .await
                .map_err(LuaError::external)?;
            match r {
                Ok(()) => Ok(LuaNil),
                Err(e) => Ok(lua.to_value(&e.to_string())?),
            }
        });

        methods.add_method_mut("stop", |_, this, ()| {
            this.0.lock().unwrap().stop();
            Ok(())
        });

        // model
        methods.add_method_mut(
            "enable_model",
            |_, this, (index, args): (LuaUserDataRef<'lua, UuidWrapper>, Vec<String>)| {
                Ok(this
                    .0
                    .lock()
                    .unwrap()
                    .enable_model(index.inner(), &args)
                    .map_err(LuaError::external)?)
            },
        );

        methods.add_method_mut(
            "disable_model",
            |_, this, index: LuaUserDataRef<'lua, UuidWrapper>| {
                Ok(this
                    .0
                    .lock()
                    .unwrap()
                    .disable_model(index.inner())
                    .map_err(LuaError::external)?)
            },
        );

        methods.add_method_mut(
            "get_model_state",
            |lua, this, index: LuaUserDataRef<'lua, UuidWrapper>| {
                Ok(lua.to_value(
                    &this
                        .0
                        .lock()
                        .unwrap()
                        .get_model_state(index.inner())
                        .map_err(LuaError::external)?,
                ))
            },
        );

        // input and output
        methods.add_async_method_mut(
            "set_controller",
            |_lua, this, (plane_id, buffer): (LuaUserDataRef<'lua, UuidWrapper>, usize)| async move{
                let controller = this
                    .0
                    .lock()
                    .unwrap()
                    .set_controller(plane_id.inner(), buffer)
                    .await.map_err(LuaError::external)?;
                Ok(InputSenderWrapper::from(controller))
            },
        );

        methods.add_method(
            "subscribe_plane",
            |_lua, this, plane_id: LuaUserDataRef<'lua, UuidWrapper>| {
                let viewer = this
                    .0
                    .lock()
                    .unwrap()
                    .subscribe_plane(plane_id.inner())
                    .map_err(LuaError::external)?;
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
            |lua,
             this,
             (model_id, plane_id, init_cfg): (
                LuaUserDataRef<'lua, UuidWrapper>,
                Option<LuaUserDataRef<'lua, UuidWrapper>>,
                LuaValue,
            )| async move {
                let init_cfg: PlaneInitCfg = lua.from_value(init_cfg)?;
                let res = this
                    .0
                    .lock()
                    .unwrap()
                    .push_plane(model_id.inner(), plane_id.map(|p| p.inner()), init_cfg)
                    .await
                    .map_err(LuaError::external)?;
                let t = lua.create_table()?;
                t.push(UuidWrapper::from(res.0))?;
                t.push(OutputReceiverWrapper::from(res.1))?;
                t.into_lua(lua)
            },
        );

        methods.add_async_method_mut(
            "remove_plane",
            |_lua, this, id: LuaUserDataRef<'lua, UuidWrapper>| async move {
                this.0.lock().unwrap().remove_plane(id.inner()).await;
                Ok(())
            },
        );

        methods.add_method(
            "contains_plane",
            |_, this, plane_id: LuaUserDataRef<'lua, UuidWrapper>| {
                this.0
                    .lock()
                    .unwrap()
                    .contains_plane(plane_id.inner())
                    .map_err(LuaError::external)
            },
        );

        // time
        methods.add_async_method("get_time", |lua, this, ()| async move {
            let time = this
                .0
                .lock()
                .unwrap()
                .get_time()
                .await
                .map_err(LuaError::external)?;
            Ok(time.as_millis().into_lua(lua))
        });

        methods.add_async_method("pause", |_lua, this, ()| async move {
            let _ = this.0.lock().unwrap().pause().await;
            Ok(())
        });

        methods.add_async_method("resume", |_lua, this, ()| async move {
            let _ = this.0.lock().unwrap().pause().await;
            Ok(())
        });

        methods.add_method("clone", |_lua, this, ()| Ok(this.clone()));
    }
}
