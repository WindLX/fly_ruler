/// fatal error which may cause critical problem and must be transmit to the fly_ruler system level
#[derive(Debug)]
pub enum FrError {
    /// The system has been used in an unsupported way
    Unsupported(String),
    /// An unexpected bug has happened, please contact the author
    ReportableBug(String),
    /// A read or write error has happened when interacting with file system
    Io(std::io::Error),
    /// Error cause by fly_ruler_core
    // Core(FatalCoreError),
    /// Error caused by fly_ruler_plugin
    Plugin(FatalPluginError),
    /// a failpoint has been triggered for testing purposes
    #[doc(hidden)]
    #[cfg(feature = "failpoints")]
    FailPoint,
}

impl std::error::Error for FrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            // Self::Core(e) => Some(e),
            Self::Plugin(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for FrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported(s) => write!(f, "Unsupported: {}", s),
            Self::ReportableBug(s) => write!(f, "ReportableBug: {}", s),
            Self::Io(e) => write!(f, "Io: {}", e),
            // Self::Core(e) => write!(f, "Core: {}", e),
            Self::Plugin(e) => write!(f, "Plugin: {}", e),
            #[cfg(feature = "failpoints")]
            Self::FailPoint => write!(f, "FailPoint"),
        }
    }
}

/// fatal error which occured in fly_ruler_core
#[derive(Debug)]
pub enum FatalCoreError {}

#[derive(Debug)]
pub struct FatalPluginError {
    pub name: String,
    pub result: i32,
    pub reason: String,
}

impl FatalPluginError {
    pub fn new(name: &str, result: i32, reason: &str) -> Self {
        Self {
            name: name.to_string(),
            result,
            reason: reason.to_string(),
        }
    }
}

impl std::error::Error for FatalPluginError {}

impl std::fmt::Display for FatalPluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "plugin {} failed with code {}: {}",
            self.name, self.result, self.reason
        )
    }
}
