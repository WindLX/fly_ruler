use super::ffi::FrModelGetState;
use crate::plugin::{IsPlugin, Plugin, PluginError};
use fly_ruler_utils::error::FatalPluginError;
use fly_ruler_utils::Vector;
use libc::c_double;
use std::path::Path;

pub type ModelGetStateFn = dyn Fn(&Vector) -> Result<Vector, FatalPluginError>;

#[derive(Debug)]
pub struct Model {
    plugin: Plugin,
}

impl Model {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, PluginError> {
        let plugin = Plugin::new(path)?;
        Ok(Model { plugin })
    }

    pub fn get_state_handler(&self) -> Result<FrModelGetState, FatalPluginError> {
        let get_state = self
            .load_function::<FrModelGetState>("frmodel_get_state")
            .map_err(|e| FatalPluginError::new(&self.info().name, -2, &e.to_string()))?;
        Ok(*get_state)
    }
}

pub fn get_state_handler_constructor(
    handler: FrModelGetState,
    name: String,
) -> Box<dyn Fn(&Vector) -> Result<Vector, FatalPluginError>> {
    let name = name.clone();
    let h = move |xu: &Vector| {
        let xu = &xu.data;
        let mut xdot = [0.0; 18];
        let xdot_ptr = xdot.as_mut_ptr() as *mut f64;
        unsafe {
            let res = handler(xu.as_ptr() as *const c_double, xdot_ptr);
            if res < 0 {
                return Err(FatalPluginError::new(
                    &name,
                    res,
                    "when call frmodel_get_state",
                ));
            } else {
                let xdot = std::slice::from_raw_parts_mut(xdot_ptr, 18).to_vec();
                Ok(Vector::from(xdot))
            }
        }
    };
    // Rc::new(RefCell::new(h))
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
    use crate::model::get_state_handler_constructor;
    use crate::plugin::plugin::IsPlugin;
    use fly_ruler_utils::logger::test_init;
    use fly_ruler_utils::model::Vector;

    #[test]
    fn test_model_info() {
        /*
         *  description = "A f16 model based on NASA reports"
         *  name = "f16_model"
         *  author = "windlx"
         *  version = "0.1.0"
         */
        test_init();
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
        test_init();
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
        test_init();
        let model = Model::new("./install");
        assert!(matches!(model, Ok(_)));
        let model = model.unwrap();
        let res = model.plugin().install(vec![Box::new("./data")]);
        assert!(matches!(res, Ok(Ok(_))));
        let res = model.get_state_handler();
        let d = Vector::from(vec![
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
            2109.41286903712,
            -2.24414978017729,
            -0.0935778861396136,
            0.0944687551889544,
            6.28161378774449,
            1.0,
        ]);
        let h = res.unwrap();
        let f = get_state_handler_constructor(h, model.info().name.clone());
        let r = f(&d);
        dbg!(&r);
        let res = model.plugin().uninstall(Vec::new());
        assert!(matches!(res, Ok(Ok(_))));
    }
}
