use crate::manager::{AsPluginManager, ModelManager};
use fly_ruler_core::{
    core::{Core, CoreInitCfg, PlaneInitCfg},
    parts::trim::TrimOutput,
};
use fly_ruler_plugin::{PluginInfo, PluginState};
use fly_ruler_utils::{
    error::{FrError, FrResult},
    CancellationToken, InputSender, OutputReceiver,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
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
    ) -> Result<
        (
            Uuid,
            OutputReceiver,
            InputSender,
            JoinHandle<FrResult<()>>,
            TrimOutput,
        ),
        SysError,
    > {
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
