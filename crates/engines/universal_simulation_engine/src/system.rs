use crate::manager::{AsPluginManager, ModelManager};
use fly_ruler_core::core::{Core, CoreInitCfg, PlaneInitCfg};
use fly_ruler_plugin::{PluginInfo, PluginState};
use fly_ruler_utils::{error::FrError, input_channel, InputSender, OutputReceiver};
use log::{error, info, trace};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Duration,
};
use thiserror::Error;
use uuid::Uuid;

pub struct System {
    model_root: PathBuf,
    model_manager: Option<ModelManager>,
    core: Option<Core>,
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

    pub fn enable_model(&mut self, model_id: Uuid, args: &[impl ToString]) -> Result<(), SysError> {
        if let Some(manager) = &mut self.model_manager {
            manager.enable(model_id, args)?;
            Ok(())
        } else {
            Err(SysError::ManagerNotInit)
        }
    }

    pub fn disable_model(&mut self, model_id: Uuid) -> Result<(), SysError> {
        if let Some(manager) = &mut self.model_manager {
            manager.disable(model_id)?;
            Ok(())
        } else {
            Err(SysError::ManagerNotInit)
        }
    }

    pub fn get_model_state(&self, model_id: Uuid) -> Result<Option<PluginState>, SysError> {
        if let Some(manager) = &self.model_manager {
            let r = manager.state(model_id);
            Ok(r)
        } else {
            Err(SysError::ManagerNotInit)
        }
    }

    pub fn init(&mut self, init_cfg: CoreInitCfg) {
        self.model_manager = Some(ModelManager::new(&self.model_root));
        let core = Core::new(init_cfg);
        self.core = Some(core);
    }

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

    pub fn subscribe_plane(&self, plane_id: Uuid) -> Result<Option<OutputReceiver>, SysError> {
        match &self.core {
            Some(core) => Ok(core.subscribe_plane(plane_id)),
            None => Err(SysError::CoreNotInit),
        }
    }

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

    pub async fn step(&mut self, is_block: bool) -> Result<Result<(), FrError>, SysError> {
        match &mut self.core {
            Some(core) => {
                let r = core.step(is_block).await;
                trace!("system step task fininshed");
                Ok(r)
            }
            None => Err(SysError::CoreNotInit),
        }
    }

    pub async fn pause(&self) -> Result<(), SysError> {
        match &self.core {
            Some(core) => {
                core.pause().await;
                info!("core has been paused");
                Ok(())
            }
            None => Err(SysError::CoreNotInit),
        }
    }

    pub async fn resume(&self) -> Result<(), SysError> {
        match &self.core {
            Some(core) => {
                core.resume().await;
                info!("core has been resumed");
                Ok(())
            }
            None => Err(SysError::CoreNotInit),
        }
    }

    pub fn err_stop(&mut self) {
        let p = self.model_manager.as_mut().unwrap();
        let _ = p.disable_all();
    }

    pub fn stop(&mut self) {
        let p = self.model_manager.as_mut().unwrap();
        let _ = p.disable_all();
        info!("system exited successfully");
    }

    pub fn contains_plane(&self, plane_id: Uuid) -> Result<bool, SysError> {
        match &self.core {
            Some(core) => Ok(core.contains_plane(plane_id)),
            None => Err(SysError::CoreNotInit),
        }
    }

    pub fn planes(&self) -> Result<Vec<Uuid>, SysError> {
        match &self.core {
            Some(core) => Ok(core.planes()),
            None => Err(SysError::CoreNotInit),
        }
    }

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
