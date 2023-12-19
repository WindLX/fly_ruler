use super::ffi::{FrModelLoadConstants, FrModelStep, PlantConstants, C};
use crate::plugin::{IsPlugin, Plugin, PluginError};
use fly_ruler_utils::error::FatalPluginError;
use fly_ruler_utils::plant_model::ModelInput;
use std::path::Path;

pub type ModelStepFn = dyn Fn(&ModelInput) -> Result<C, FatalPluginError>;

#[derive(Debug)]
pub struct Model {
    plugin: Plugin,
}

impl Model {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, PluginError> {
        let plugin = Plugin::new(path)?;
        Ok(Model { plugin })
    }

    pub fn load_constants(&self) -> Result<PlantConstants, FatalPluginError> {
        let load_constants = self
            .load_function::<FrModelLoadConstants>("frmodel_load_constants")
            .map_err(|e| FatalPluginError::symbol(e.to_string()))?;
        let mut constants = Box::new(PlantConstants::default());
        let constants_ptr = &mut *constants;
        unsafe {
            let res = load_constants(constants_ptr);
            if res < 0 {
                return Err(FatalPluginError::inner(
                    &self.info().name,
                    res,
                    "when call frmodel_load_constants",
                ));
            } else {
                let constants = *constants_ptr;
                Ok(constants)
            }
        }
    }

    pub fn get_step_handler(&self) -> Result<FrModelStep, FatalPluginError> {
        let step = self
            .load_function::<FrModelStep>("frmodel_step")
            .map_err(|e| FatalPluginError::symbol(e.to_string()))?;
        Ok(*step)
    }
}

pub fn step_handler_constructor(
    handler: FrModelStep,
    name: String,
) -> Box<dyn Fn(&ModelInput) -> Result<C, FatalPluginError>> {
    let name = name.clone();
    let h = move |input: &ModelInput| {
        let state = Box::new(input.state);
        let control = Box::new(input.control);
        let mut c = Box::new(C::default());
        let c_ptr = &mut *c;
        unsafe {
            let res = handler(&*state, &*control, input.lef, c_ptr);
            if res < 0 {
                return Err(FatalPluginError::inner(
                    &name,
                    res,
                    "when call frmodel_step",
                ));
            } else {
                let c = *c_ptr;
                Ok(c)
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
    use crate::model::ffi::PlantConstants;
    use crate::model::step_handler_constructor;
    use crate::plugin::plugin::IsPlugin;
    use fly_ruler_utils::logger::test_logger_init;
    use fly_ruler_utils::plant_model::ModelInput;
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
    fn test_model_load_constants() {
        test_logger_init();
        let model = Model::new("./install");
        assert!(matches!(model, Ok(_)));
        let model = model.unwrap();
        let res = model.plugin().install(vec![Box::new("./data")]);
        assert!(matches!(res, Ok(Ok(_))));

        let constants = model.load_constants();
        assert!(matches!(constants, Ok(_)));
        assert_eq!(
            constants.unwrap(),
            PlantConstants::new(
                636.94, 30.0, 300.0, 11.32, 0.35, 0.30, 0.0, 55814.0, 982.0, 63100.0, 9496.0
            )
        );

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
