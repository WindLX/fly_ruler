use crate::manager::{AsPluginManager, ModelManager};
use fly_ruler_core::core::{Core, CoreInitCfg, PlaneInitCfg};
use fly_ruler_plugin::{PluginInfo, PluginState};
use fly_ruler_utils::error::FrResult;
use fly_ruler_utils::CancellationToken;
use fly_ruler_utils::{error::FrError, InputSender, OutputReceiver};
use lua_runtime::{prelude::*, CancellationTokenWrapper};
use lua_runtime::{InputSenderWrapper, OutputReceiverWrapper, UuidWrapper};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Mutex,
};
use thiserror::Error;
use tokio::task::JoinHandle;
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

    #[instrument(skip(self, init_cfg, cancellation_token), level = Level::INFO, err)]
    pub fn push_plane(
        &mut self,
        model_id: Uuid,
        controller_buffer: usize,
        init_cfg: PlaneInitCfg,
        cancellation_token: CancellationToken,
    ) -> Result<(Uuid, OutputReceiver, InputSender, JoinHandle<FrResult<()>>), SysError> {
        let model = if let Some(manager) = &mut self.model_manager {
            manager.get_model(model_id)
        } else {
            return Err(SysError::ManagerNotInit);
        };
        match model {
            Some(model) => match &mut self.core {
                Some(core) => {
                    Ok(core.push_plane(model, controller_buffer, init_cfg, cancellation_token)?)
                }
                None => Err(SysError::CoreNotInit),
            },
            None => Err(SysError::ModelNotAvailable),
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

        // plane
        methods.add_method_mut(
            "push_plane",
            |lua,
             this,
             (model_id, controller_buffer, init_cfg): (
                LuaUserDataRef<'lua, UuidWrapper>,
                usize,
                LuaValue,
            )| {
                let cancellation_token = CancellationToken::new();
                let init_cfg: PlaneInitCfg = lua.from_value(init_cfg)?;
                let (id, viewer, controller, _handler) = this
                    .0
                    .lock()
                    .unwrap()
                    .push_plane(
                        model_id.inner(),
                        controller_buffer,
                        init_cfg,
                        cancellation_token.clone(),
                    )
                    .map_err(LuaError::external)?;
                let t = lua.create_table()?;
                t.push(UuidWrapper::from(id))?;
                t.push(OutputReceiverWrapper::from(viewer))?;
                t.push(InputSenderWrapper::from(controller))?;
                t.push(CancellationTokenWrapper::from(cancellation_token))?;
                t.into_lua(lua)
            },
        );

        methods.add_method("clone", |_lua, this, ()| Ok(this.clone()));
    }
}
