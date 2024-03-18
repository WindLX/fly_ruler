use crate::manager::{AsPluginManager, ModelManager};
use fly_ruler_core::core::{Core, CoreInitCfg, PlaneInitCfg};
use fly_ruler_plugin::{PluginInfo, PluginState};
use fly_ruler_utils::{
    error::{FatalCoreError, FrError},
    input_channel, InputSender, OutputReceiver,
};
use log::{error, info, trace, warn};
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
        if let Some(manager) = &self.model_manager {
            let infos = manager.infos();
            let states = manager.states();
            infos
                .iter()
                .zip(states.iter())
                .map(|(info, state)| (*info.0, (info.1.clone(), state.1.clone())))
                .collect()
        } else {
            warn!("Sys: model manager is not initialized");
            HashMap::new()
        }
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
        let model = if let Some(manager) = &mut self.model_manager {
            manager.get_model(model_id)
        } else {
            warn!("Sys: model manager is not initialized");
            None
        };
        match model {
            Some(model) => match &mut self.core {
                Some(core) => {
                    let state_receiver = core.push_plane(model, init_cfg).await;
                    match state_receiver {
                        Ok(state_receiver) => Some(state_receiver),
                        Err(e) => {
                            error!("{}", e);
                            None
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

    pub async fn step(&mut self) -> Option<()> {
        match &mut self.core {
            Some(core) => {
                let output = core.step().await;
                match output {
                    Err(e) => {
                        if let FrError::Sync(_) | FrError::Core(FatalCoreError::Controller(_)) = e {
                            warn!("{}", e);
                            Some(())
                        } else {
                            error!("{}", e);
                            None
                        }
                    }
                    Ok(_) => {
                        trace!("system step task fininshed");
                        Some(())
                    }
                }
            }
            None => {
                warn!("Sys: core is not initialized");
                None
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

    pub fn err_stop(&mut self) -> ! {
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
