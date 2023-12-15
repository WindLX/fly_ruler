use crate::plugin::{PluginError, PluginInfo};
use fly_ruler_utils::FrError;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct CModelError {
    pub info: PluginInfo,
    pub message: CModelErrorMessage,
}

#[derive(Debug, Clone)]
pub enum CModelErrorMessage {
    CModelLoadError(String),
    CModelInstallError(String),
    CModelUninstallError(String),
    CModelGetStateError(String),
}

impl PluginError for CModelError {
    fn info(&self) -> &PluginInfo {
        self.info()
    }
}

impl FrError for CModelError {
    fn log_level(&self) -> log::Level {
        log::Level::Warn
    }
}

impl Error for CModelError {}

impl Display for CModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.message {
            CModelErrorMessage::CModelLoadError(path) => {
                write!(
                    f,
                    "ModelLoadError: failed to load model `{}` at path of {}",
                    self.info.name, path
                )
            }
            CModelErrorMessage::CModelInstallError(message) => {
                write!(
                    f,
                    "ModelInstallError: fail to load model `{}` due to {}",
                    self.info.name, message
                )
            }
            CModelErrorMessage::CModelUninstallError(message) => {
                write!(
                    f,
                    "ModelUninstallError: fail to load model `{}` due to {}",
                    self.info.name, message
                )
            }
            CModelErrorMessage::CModelGetStateError(message) => {
                write!(
                    f,
                    "ModelGetStateError: fail to load model `{}` due to {}",
                    self.info.name, message
                )
            }
        }
    }
}
