pub(crate) mod model;
pub(crate) mod plugin;

pub use model::{
    init_handler_constructor, step_handler_constructor, trim_handler_constructor, AerodynamicModel,
    AerodynamicModelInitFn, AerodynamicModelStepFn, AerodynamicModelTrimFn,
};
pub use plugin::{AsPlugin, PluginError, PluginInfo, PluginState};
