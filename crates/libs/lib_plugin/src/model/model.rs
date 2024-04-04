use super::ffi::{
    FrModelInit, FrModelLoadConstants, FrModelLoadCtrlLimits, FrModelStep, FrModelTrim,
};
use crate::plugin::{AsPlugin, Plugin, PluginError};
use fly_ruler_utils::error::FatalPluginError;
use fly_ruler_utils::plane_model::{ControlLimit, MechanicalModelInput, PlaneConstants, C};
use std::path::Path;
use tracing::{event, instrument, span, Level};

pub type AerodynamicModelTrimFn = dyn Fn(&MechanicalModelInput) -> Result<C, FatalPluginError>;
pub type AerodynamicModelInitFn = dyn Fn(&MechanicalModelInput) -> Result<(), FatalPluginError>;
pub type AerodynamicModelStepFn = dyn Fn(&MechanicalModelInput, f64) -> Result<C, FatalPluginError>;

#[derive(Debug)]
pub struct AerodynamicModel {
    plugin: Plugin,
}

impl AerodynamicModel {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, PluginError> {
        let s = span!(Level::TRACE, "Model Create", path = %path.as_ref().display());
        let _ = s.enter();
        let plugin = Plugin::new(path)?;
        Ok(AerodynamicModel { plugin })
    }

    #[instrument(skip(self), level = Level::TRACE)]
    pub fn load_constants(&self) -> Result<PlaneConstants, FatalPluginError> {
        let load_constants = self
            .load_function::<FrModelLoadConstants>("frmodel_load_constants")
            .map_err(|e| FatalPluginError::symbol(e.to_string()))?;
        let mut constants = Box::new(PlaneConstants::default());
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
                event!(Level::DEBUG, "Plane Constants:\n{}", constants);
                Ok(constants)
            }
        }
    }

    #[instrument(skip(self), level = Level::TRACE)]
    pub fn load_ctrl_limits(&self) -> Result<ControlLimit, FatalPluginError> {
        let load_ctrl_limits = self
            .load_function::<FrModelLoadCtrlLimits>("frmodel_load_ctrl_limits")
            .map_err(|e| FatalPluginError::symbol(e.to_string()))?;
        let mut ctrl_limits = Box::new(ControlLimit::default());
        let ctrl_limits_ptr = &mut *ctrl_limits;
        unsafe {
            let res = load_ctrl_limits(ctrl_limits_ptr);
            if res < 0 {
                Err(FatalPluginError::inner(
                    &self.info().name,
                    res,
                    "when call frmodel_load_ctrl_limits",
                ))
            } else {
                let ctrl_limits = *ctrl_limits_ptr;
                event!(Level::DEBUG, "Ctrl Limits:\n{}", ctrl_limits);
                Ok(ctrl_limits)
            }
        }
    }

    pub fn get_trim_handler(&self) -> Result<FrModelTrim, FatalPluginError> {
        let init = self
            .load_function::<FrModelTrim>("frmodel_trim")
            .map_err(|e| FatalPluginError::symbol(e.to_string()))?;
        Ok(*init)
    }

    pub fn get_init_handler(&self) -> Result<FrModelInit, FatalPluginError> {
        let init = self
            .load_function::<FrModelInit>("frmodel_init")
            .map_err(|e| FatalPluginError::symbol(e.to_string()))?;
        Ok(*init)
    }

    pub fn get_step_handler(&self) -> Result<FrModelStep, FatalPluginError> {
        let step = self
            .load_function::<FrModelStep>("frmodel_step")
            .map_err(|e| FatalPluginError::symbol(e.to_string()))?;
        Ok(*step)
    }
}

pub fn init_handler_constructor(
    handler: FrModelInit,
    name: String,
) -> Box<dyn Fn(&MechanicalModelInput) -> Result<(), FatalPluginError>> {
    let name = name.clone();
    let h = move |input: &MechanicalModelInput| {
        let state = Box::new(input.state);
        let control = Box::new(input.control);
        unsafe {
            let res = handler(&*state, &*control);
            if res < 0 {
                return Err(FatalPluginError::inner(
                    &name,
                    res,
                    "when call frmodel_init",
                ));
            } else {
                Ok(())
            }
        }
    };
    Box::new(h)
}

pub fn trim_handler_constructor(
    handler: FrModelTrim,
    name: String,
) -> Box<dyn Fn(&MechanicalModelInput) -> Result<C, FatalPluginError>> {
    let name = name.clone();
    let h = move |input: &MechanicalModelInput| {
        let state = Box::new(input.state);
        let control = Box::new(input.control);
        let mut c = Box::new(C::default());
        let c_ptr = &mut *c;
        unsafe {
            let res = handler(&*state, &*control, c_ptr);
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

pub fn step_handler_constructor(
    handler: FrModelStep,
    name: String,
) -> Box<dyn Fn(&MechanicalModelInput, f64) -> Result<C, FatalPluginError>> {
    let name = name.clone();
    let h = move |input: &MechanicalModelInput, t: f64| {
        let state = Box::new(input.state);
        let control = Box::new(input.control);
        let mut c = Box::new(C::default());
        let c_ptr = &mut *c;
        unsafe {
            let res = handler(&*state, &*control, t, c_ptr);
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

impl AsPlugin for AerodynamicModel {
    fn plugin(&self) -> &Plugin {
        &self.plugin
    }

    fn plugin_mut(&mut self) -> &mut Plugin {
        &mut self.plugin
    }
}

#[cfg(test)]
mod plugin_model_tests {
    use super::AerodynamicModel;
    use crate::model::step_handler_constructor;
    use crate::plugin::plugin::AsPlugin;
    use fly_ruler_utils::logger::{debug, test_logger_init};
    use fly_ruler_utils::plane_model::{MechanicalModelInput, PlaneConstants};

    #[test]
    fn test_model_info() {
        /*
         *  description = "A f16 model based on NASA reports"
         *  name = "f16_model"
         *  author = "windlx"
         *  version = "0.1.0"
         */
        test_logger_init();
        let model = AerodynamicModel::new("../../../modules/model/f16_model");
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
        let model = AerodynamicModel::new("../../../modules/model/f16_model");
        assert!(matches!(model, Ok(_)));
        let model = model.unwrap();
        let res = model
            .plugin()
            .install(&["../../../modules/model/f16_model/data"]);
        assert!(matches!(res, Ok(Ok(_))));
        let res = model.plugin().uninstall();
        assert!(matches!(res, Ok(Ok(_))));
    }

    #[test]
    fn test_model_load_constants() {
        test_logger_init();
        let model = AerodynamicModel::new("../../../modules/model/f16_model");
        assert!(matches!(model, Ok(_)));
        let model = model.unwrap();
        let res = model
            .plugin()
            .install(&["../../../modules/model/f16_model/data"]);
        assert!(matches!(res, Ok(Ok(_))));

        let constants = model.load_constants();
        assert!(matches!(constants, Ok(_)));
        assert_eq!(
            constants.unwrap(),
            PlaneConstants::new(
                636.94, 30.0, 300.0, 11.32, 0.35, 0.30, 0.0, 55814.0, 982.0, 63100.0, 9496.0
            )
        );

        let res = model.plugin().uninstall();
        assert!(matches!(res, Ok(Ok(_))));
    }

    #[test]
    fn test_model_step() {
        test_logger_init();

        let model = AerodynamicModel::new("../../../modules/model/f16_model");
        assert!(matches!(model, Ok(_)));

        let model = model.unwrap();
        let res = model
            .plugin()
            .install(&["../../../modules/model/f16_model/data"]);
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
        // let d_lef = 6.28161378774449;
        let h = res.unwrap();
        let f = step_handler_constructor(h, model.info().name.clone());
        let r = f(&MechanicalModelInput::new(state, control), 0.0);
        debug!("{:?}", &r);

        let res = model.plugin().uninstall();
        assert!(matches!(res, Ok(Ok(_))));
    }
}
