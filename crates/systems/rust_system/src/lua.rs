use std::path::Path;

use fly_ruler_core::core::{CoreInitCfg, PlaneInitCfg};
use log::error;
use mlua::prelude::*;
use serde::de::DeserializeOwned;

pub struct LuaManager {
    lua: Lua,
}

impl LuaManager {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let lua = Lua::new();
        lua.load(path.as_ref())
            .exec()
            .map_err(|e| {
                error!("{}", e);
                std::process::exit(1);
            })
            .unwrap();
        LuaManager { lua }
    }

    fn load_data<'lua, T>(&'lua self, key: &str) -> T
    where
        T: FromLua<'lua>,
    {
        self.lua
            .globals()
            .get(key)
            .map_err(|e| {
                error!("{}", e);
                std::process::exit(1);
            })
            .unwrap()
    }

    fn load_ser_data<'lua, T>(&'lua self, key: &str) -> T
    where
        T: DeserializeOwned,
    {
        let val = self
            .lua
            .globals()
            .get(key)
            .map_err(|e| {
                error!("{}", e);
                std::process::exit(1);
            })
            .unwrap();
        self.lua
            .from_value(val)
            .map_err(|e| {
                error!("{}", e);
                std::process::exit(1);
            })
            .unwrap()
    }

    pub fn server_addr(&self) -> String {
        let server_addr: String = self.load_data("server_addr");
        server_addr
    }

    pub fn tick_timeout(&self) -> u64 {
        let tick_timeout: u64 = self.load_data("tick_timeout");
        tick_timeout
    }

    pub fn is_block(&self) -> bool {
        let is_block: bool = self.load_data("is_block");
        is_block
    }

    pub fn core_init_cfg(&self) -> CoreInitCfg {
        let cfg: CoreInitCfg = self.load_ser_data("core_init_cfg");
        cfg
    }

    pub fn model_root_path(&self) -> String {
        let model_root_path: String = self.load_data("model_root_path");
        model_root_path
    }

    pub fn model_install_args(&self) -> Vec<Vec<String>> {
        let args: Vec<Vec<String>> = self.load_ser_data("model_install_args");
        args
    }

    pub fn plane_init_cfg(&self) -> PlaneInitCfg {
        let cfg: PlaneInitCfg = self.load_ser_data("plane_init_cfg");
        cfg
    }
}
