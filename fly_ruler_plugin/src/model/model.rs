use super::ffi::FrModelStep;
use crate::plugin::{IsPlugin, Plugin, PluginError};
use fly_ruler_utils::error::FatalPluginError;
use fly_ruler_utils::model::{ModelInput, ModelOutput};
use libc::c_double;
use std::path::Path;

pub type ModelStepFn = dyn Fn(&ModelInput) -> Result<ModelOutput, FatalPluginError>;

#[derive(Debug)]
pub struct Model {
    plugin: Plugin,
}

impl Model {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, PluginError> {
        let plugin = Plugin::new(path)?;
        Ok(Model { plugin })
    }

    pub fn get_step_handler(&self) -> Result<FrModelStep, FatalPluginError> {
        let step = self
            .load_function::<FrModelStep>("frmodel_step")
            .map_err(|e| FatalPluginError::new(&self.info().name, -2, &e.to_string()))?;
        Ok(*step)
    }
}

pub fn step_handler_constructor(
    handler: FrModelStep,
    name: String,
) -> Box<dyn Fn(&ModelInput) -> Result<ModelOutput, FatalPluginError>> {
    let name = name.clone();
    let h = move |input: &ModelInput| {
        let state = input.state.as_slice();
        let control = input.control.as_slice();
        let mut state_dot = [0.0; 12];
        let mut state_extend = [0.0; 6];
        let state_dot_ptr = state_dot.as_mut_ptr() as *mut c_double;
        let state_extend_ptr = state_extend.as_mut_ptr() as *mut c_double;
        unsafe {
            let res = handler(
                state.as_ptr() as *const c_double,
                control.as_ptr() as *const c_double,
                input.lef,
                state_dot_ptr,
                state_extend_ptr,
            );
            if res < 0 {
                return Err(FatalPluginError::new(&name, res, "when call frmodel_step"));
            } else {
                let state_dot = std::slice::from_raw_parts(state_dot_ptr, 12);
                let state_extend = std::slice::from_raw_parts(state_extend_ptr, 6);
                Ok(ModelOutput::new(state_dot, state_extend))
            }
        }
    };
    Box::new(h)
}

impl IsPlugin for Model {
    fn plugin(&self) -> &Plugin {
        &self.plugin
    }

    fn plugin_mut(&mut self) -> &mut Plugin {
        &mut self.plugin
    }
}

#[cfg(test)]
mod plugin_model_tests {
    use super::Model;
    use crate::model::step_handler_constructor;
    use crate::plugin::plugin::IsPlugin;
    use fly_ruler_utils::logger::test_logger_init;
    use fly_ruler_utils::model::ModelInput;
    use log::debug;

    #[test]
    fn test_model_info() {
        /*
         *  description = "A f16 model based on NASA reports"
         *  name = "f16_model"
         *  author = "windlx"
         *  version = "0.1.0"
         */
        test_logger_init();
        let model = Model::new("./install");
        assert!(matches!(model, Ok(_)));
        let model = model.unwrap();
        let info = model.info();
        assert_eq!(info.name, "f16_model");
        assert_eq!(info.description, "A f16 model based on NASA reports");
        assert_eq!(info.author, "windlx");
        assert_eq!(info.version, "0.1.0");
    }

    #[test]
    fn test_model_load() {
        test_logger_init();
        let model = Model::new("./install");
        assert!(matches!(model, Ok(_)));
        let model = model.unwrap();
        let res = model.plugin().install(vec![Box::new("./data")]);
        assert!(matches!(res, Ok(Ok(_))));
        let res = model.plugin().uninstall(Vec::new());
        assert!(matches!(res, Ok(Ok(_))));
    }

    #[test]
    fn test_model_step() {
        test_logger_init();

        let model = Model::new("./install");
        assert!(matches!(model, Ok(_)));

        let model = model.unwrap();
        let res = model.plugin().install(vec![Box::new("./data")]);
        assert!(matches!(res, Ok(Ok(_))));

        let res = model.get_step_handler();
        let state = [
            0.0,
            0.0,
            15000.0,
            0.0,
            0.0790758040827099,
            0.0,
            500.0,
            0.0790758040827099,
            0.0,
            0.0,
            0.0,
            0.0,
        ];
        let control = [
            2109.41286903712,
            -2.24414978017729,
            -0.0935778861396136,
            0.0944687551889544,
        ];
        let d_lef = 6.28161378774449;
        let h = res.unwrap();
        let f = step_handler_constructor(h, model.info().name.clone());
        let r = f(&ModelInput::new(state, control, d_lef));
        debug!("{:?}", &r);

        let res = model.plugin().uninstall(Vec::new());
        assert!(matches!(res, Ok(Ok(_))));
    }
}
