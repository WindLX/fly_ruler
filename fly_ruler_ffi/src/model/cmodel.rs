use super::error::CModelError;
use crate::plugin::ffi::{FrPluginInstallHook, FrPluginRegisterLogger, Logger};
use crate::plugin::{Plugin, PluginInfo};
use fly_ruler_utils::FrError;
use libc::{c_char, c_double, c_int};
use libloading::{Error, Library, Symbol};
use std::ffi::{CStr, CString};
use std::path::Path;

#[derive(Debug)]
pub struct CModel {
    info: PluginInfo,
    lib: Library,
}

impl CModel {
    pub fn new(path: &str) -> Option<Self> {
        let model_path = Path::new(path);
        let info_path = model_path.join("info.toml");
        if !model_path.exists() || !info_path.exists() {
            return None;
        }
        let info = PluginInfo::load(&info_path)?;
        let lib_name = if cfg!(target_os = "linux") {
            format!("lib{}.so", info.name)
        } else if cfg!(target_os = "windows") {
            format!("{}.dll", info.name)
        } else {
            return None;
        };
        let lib_path = model_path.join(lib_name);
        unsafe {
            let lib = Library::new(lib_path).ok()?;
            Some(CModel { info, lib })
        }
    }

    pub fn model_install(&self, arg: &str) -> Result<(), CModelError> {
        unsafe {
            let func = self.get_install_hook();
            match func {
                Ok(func) => {
                    let res = func(1, arg);
                    if res < 0 {
                        Err(CModelError {
                            info: self.info().clone(),
                            message: super::error::CModelErrorMessage::CModelInstallError(format!(
                                "some error occurred when model install and return code is `{res}`"
                            )),
                        })
                    } else {
                        Ok(())
                    }
                }
                Err(err) => Err(CModelError {
                    info: err.info().clone(),
                    message: super::error::CModelErrorMessage::CModelInstallError(String::from(
                        "failed find symbol",
                    )),
                }),
            }
        }
    }

    pub fn model_uninstall(&self) -> Result<(), CModelError> {
        unsafe {
            let func = self.get_uninstall_hook();
            match func {
                Ok(func) => {
                    let res = func(0);
                    if res < 0 {
                        Err(CModelError {
                            info: self.info().clone(),
                            message: super::error::CModelErrorMessage::CModelInstallError(format!(
                                "some error occurred when model install and return code is `{res}`"
                            )),
                        })
                    } else {
                        Ok(())
                    }
                }
                Err(err) => Err(CModelError {
                    info: err.info().clone(),
                    message: super::error::CModelErrorMessage::CModelInstallError(String::from(
                        "failed find symbol",
                    )),
                }),
            }
        }
    }

    // pub fn model_get_state(&self, xu: Vec<f64>) -> Vec<f64> {}
}

impl Plugin for CModel {
    fn info(&self) -> &PluginInfo {
        &self.info
    }

    fn get_install_hook(
        &self,
    ) -> Result<crate::plugin::ffi::FrPluginInstallHook, Box<dyn crate::plugin::PluginError>> {
        unsafe {
            let func: Result<Symbol<FrPluginInstallHook>, Error> =
                self.lib.get(b"frplugin_install_hook");
            match func {
                Ok(func) => Ok(func),
                Err(err) => Err(Box::new(
                    CModelError {
                        info: err.info().clone(),
                        message: super::error::CModelErrorMessage::CModelInstallError(
                            String::from("failed find symbol"),
                        ),
                    }
                    .into(),
                )),
            }
        }
    }

    fn register_logger(&self) -> Result<(), Box<dyn FrError>> {
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
}

#[cfg(test)]
mod model_tests {
    use crate::model::callback;

    use super::CModel;

    #[test]
    fn test_model() {
        let model = CModel::new("f16", "./f16_model.dll");
        assert!(matches!(model, Some(_)));
        let model = model.unwrap();
        model.register_logger(callback);
        let a = model.model_install("./data");
        let a = model.model_uninstall();
    }
}
