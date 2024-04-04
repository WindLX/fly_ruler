use fly_ruler_core::core::{CoreInitCfg, PlaneInitCfg};
use mlua::prelude::*;
use serde::de::DeserializeOwned;
use std::path::Path;

pub struct LuaManager {
    lua: Lua,
}

impl LuaManager {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let lua = Lua::new();
        lua.load(path.as_ref())
            .exec()
            .map_err(|e| {
                panic!("{}", e);
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
                panic!("{}", e);
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
                panic!("{}", e);
            })
            .unwrap();
        self.lua
            .from_value(val)
            .map_err(|e| {
                panic!("{}", e);
            })
            .unwrap()
    }

    fn load_table_data<'lua, T>(&'lua self, table: &str, key: &str) -> T
    where
        T: FromLua<'lua>,
    {
        let table: LuaTable = self.load_data(table);
        let var: T = table
            .get(key)
            .map_err(|e| {
                println!("{}", e);
                std::process::exit(1);
            })
            .unwrap();
        var
    }

    pub fn server_addr(&self) -> String {
        let server_addr: String = self.load_table_data("server", "addr");
        server_addr
    }

    pub fn tick_timeout(&self) -> u64 {
        let tick_timeout: u64 = self.load_table_data("server", "tick_timeout");
        tick_timeout
    }

    pub fn read_rate(&self) -> u64 {
        let mut read_rate: u64 = self.load_table_data("server", "read_rate");
        read_rate = read_rate.max(1);
        read_rate
    }

    pub fn is_block(&self) -> bool {
        let is_block: bool = self.load_table_data("server", "is_block");
        is_block
    }

    pub fn core_init_cfg(&self) -> CoreInitCfg {
        let cfg: CoreInitCfg = self.load_ser_data("core_init_cfg");
        cfg
    }

    pub fn model_root_path(&self) -> String {
        let model_root_path: String = self.load_table_data("system", "model_root_path");
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

    pub fn log_filter(&self) -> String {
        let log_filter: String = self.load_table_data("log", "filter");
        log_filter
    }

    pub fn log_dir(&self) -> String {
        let log_dir: String = self.load_table_data("log", "dir");
        log_dir
    }

    pub fn log_file(&self) -> String {
        let log_file: String = self.load_table_data("log", "file");
        log_file
    }
}
