use crate::model_ffi::{FrModelGetState, FrModelInstallHook, FrModelUninstallHook};
use crate::utils_ffi::{FrUtilsRegisterLogger, InfoLevel, Logger};
use libc::c_char;
use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::path::Path;

#[derive(Debug)]
pub struct ModelLoader {
    name: String,
    path: String,
    pub lib: Option<Library>,
}

impl ModelLoader {
    pub fn new(name: &str, path: &str) -> Option<Self> {
        let t_path = Path::new(path);
        if !t_path.exists() {
            return None;
        }
        Some(Self {
            name: name.to_string(),
            path: path.to_string(),
            lib: None,
        })
    }

    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let lib = Library::new(self.path.clone())?;
            self.lib = Some(lib);
        }
        Ok(())
    }

    pub fn model_install(&self, arg: &str) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            match &self.lib {
                Some(lib) => {
                    let func: Symbol<FrModelInstallHook> = lib.get(b"frmodel_install_hook")?;
                    func(1, CString::new(arg)?.as_ptr());
                }
                None => return Ok(()),
            }
        }
        Ok(())
    }

    pub fn model_uninstall(&self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            match &self.lib {
                Some(lib) => {
                    let func: Symbol<FrModelUninstallHook> = lib.get(b"frmodel_uninstall_hook")?;
                    func(0);
                }
                None => return Ok(()),
            }
        }
        Ok(())
    }

    pub fn register_logger(&self, logger: Logger) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            match &self.lib {
                Some(lib) => {
                    let func: Symbol<FrUtilsRegisterLogger> =
                        lib.get(b"frutils_register_logger")?;
                    func(logger);
                }
                None => return Ok(()),
            }
        }
        Ok(())
    }

    // pub fn model_get_state(&self, xu: Vec<f64>) -> Vec<f64> {}
}

fn info_level_to_string(level: InfoLevel) -> &'static str {
    match level {
        InfoLevel::TRACE => "Trace",
        InfoLevel::DEBUG => "Debug",
        InfoLevel::INFO => "Info",
        InfoLevel::WARN => "Warn",
        InfoLevel::ERROR => "Error",
        InfoLevel::FATAL => "Fatal",
    }
}

pub unsafe extern "C" fn callback(msg: *const c_char, level: InfoLevel) {
    let msg = CStr::from_ptr(msg).to_str().unwrap().to_string();
    println!("[{}] {:?}", info_level_to_string(level), msg);
}

#[cfg(test)]
mod model_tests {
    use crate::model::callback;

    use super::ModelLoader;

    #[test]
    fn test_model() {
        let model = ModelLoader::new("f16", "./f16_model.dll");
        assert!(matches!(model, Some(_)));
        let model = model.unwrap();
        model.register_logger(callback);
        let a = model.model_install("./data");
        let a = model.model_uninstall();
    }
}
