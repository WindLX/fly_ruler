use std::error::Error;

/// fatal error which may cause critical problem and must be transmit to the fly_ruler system level
#[derive(Debug)]
pub enum FrError {
    /// A read or write error has happened when interacting with file system
    Io(std::io::Error),
    /// Sync error
    Sync(String),
    /// Invalid cfg format
    Cfg(String),
    /// Error cause by fly_ruler_core
    Core(FatalCoreError),
    /// Error caused by fly_ruler_plugin
    Plugin(FatalPluginError),
}

impl std::error::Error for FrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Plugin(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for FrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "Io: {}", e),
            Self::Sync(e) => write!(f, "Sync: {}", e),
            Self::Cfg(e) => write!(f, "Cfg: {}", e),
            Self::Core(e) => write!(f, "Core: {}", e),
            Self::Plugin(e) => write!(f, "Plugin: {}", e),
        }
    }
}

/// inner error which occurred in a extern plugin
#[derive(Debug)]
pub struct PluginInner {
    name: String,
    result: i32,
    reason: String,
}

impl PluginInner {
    pub(crate) fn new(name: &str, result: i32, reason: &str) -> Self {
        Self {
            name: name.to_string(),
            result,
            reason: reason.to_string(),
        }
    }
}

impl std::error::Error for PluginInner {}

impl std::fmt::Display for PluginInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "plugin {} fail with code {}: {}",
            self.name, self.result, self.reason
        )
    }
}

/// fatal error which occured in fly_ruler_core
/// Model: error occured in extern model
#[derive(Debug)]
pub enum FatalCoreError {
    Controller(String),
    Plugin(FatalPluginError),
    Nan,
}

impl FatalCoreError {}

impl std::error::Error for FatalCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Controller(_) => None,
            Self::Plugin(e) => Some(e),
            Self::Nan => None,
        }
    }
}

impl std::fmt::Display for FatalCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Controller(e) => write!(f, "controller for plane {} not found", e),
            Self::Plugin(_) => write!(f, "{}", self.source().unwrap()),
            Self::Nan => write!(f, "NaN value"),
        }
    }
}

impl From<FatalPluginError> for FatalCoreError {
    fn from(value: FatalPluginError) -> Self {
        Self::Plugin(value)
    }
}

/// fatal error which occured in fly_ruler_plugin
/// Symbol: fail to find target Symbol in dll/so
/// Inner: error occured in extern plugin
#[derive(Debug)]
pub enum FatalPluginError {
    Symbol(String),
    Inner(PluginInner),
}

impl FatalPluginError {
    pub fn symbol(msg: String) -> Self {
        Self::Symbol(msg)
    }

    pub fn inner(name: &str, result: i32, reason: &str) -> Self {
        Self::Inner(PluginInner::new(name, result, reason))
    }
}

impl std::error::Error for FatalPluginError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::Inner(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
}

impl std::fmt::Display for FatalPluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Symbol(msg) => write!(f, "(Symbol) {}", msg),
            Self::Inner(_) => write!(f, "(Inner) {}", self.source().unwrap()),
        }
    }
}
