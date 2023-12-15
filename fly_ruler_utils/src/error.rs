use std::{error::Error, fmt::Display};

pub trait FrError: Error + Send + Sync {
    fn log_level(&self) -> log::Level;
}

#[derive(Debug, Clone)]
pub enum CoreError {
    TestError,
}

impl Error for CoreError {}

impl Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TestError => write!(f, "CoreError"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SystemError {}

#[derive(Debug, Clone)]
pub enum ModelError {
    ModelSearchError(String, String),
    ModelLoadError(String, String),
    ModelInstallError(String, String),
    ModelUninstallError(String, String),
    ModelGetStateError(String, String),
}

impl Error for ModelError {}

impl Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ModelSearchError(name, version) => {
                write!(f, "ModelSearchError: {}:{}", name, version)
            }
            Self::ModelLoadError(name, version) => {
                write!(f, "ModelLoadError: {}:{}", name, version)
            }
            Self::ModelInstallError(name, version) => {
                write!(f, "ModelInstallError: {}:{}", name, version)
            }
            Self::ModelUninstallError(name, version) => {
                write!(f, "ModelUninstallError: {}:{}", name, version)
            }
            Self::ModelGetStateError(name, version) => {
                write!(f, "ModelGetStateError: {}:{}", name, version)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PluginError {}
