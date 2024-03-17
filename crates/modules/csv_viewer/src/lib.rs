use fly_ruler_utils::plane_model::CoreOutput;
use mlua::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

fn title_line() -> String {
    [
        "time(s)",
        // state
        "npos(ft)",
        "epos(ft)",
        "altitude(ft)",
        "phi(rad)",
        "theta(rad)",
        "psi(rad)",
        "velocity(ft/s)",
        "alpha(rad)",
        "beta(rad)",
        "p(rad/s)",
        "q(rad/s)",
        "r(rad/s)",
        // control
        "thrust(lbs)",
        "elevator(deg)",
        "aileron(deg)",
        "rudder(deg)",
        //d_lef
        "d_lef(deg)",
        // extend
        "nx(g)",
        "ny(g)",
        "nz(g)",
        "mach",
        "qbar(lb/ft ft)",
        "ps(lb/ft ft)",
    ]
    .join(",")
}

struct CsvWriter {
    file: File,
}

impl CsvWriter {
    fn new(path: &str) -> LuaResult<Self> {
        std::fs::write(path, format!("{}\n", title_line()))
            .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        let file = OpenOptions::new()
            .append(true)
            .open(path)
            .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        Ok(Self { file })
    }

    fn write_line(&mut self, time: f64, output: &CoreOutput) -> LuaResult<()> {
        let v: Vec<String> = std::iter::once(format!("{:.3}", time))
            .chain(
                Into::<Vec<f64>>::into(*output)
                    .iter()
                    .map(|d| d.to_string()),
            )
            .collect();
        let v = v.join(",");
        self.file
            .write_all(format!("{}\n", v).as_bytes())
            .map_err(|e| LuaError::RuntimeError(e.to_string()))
    }
}

impl LuaUserData for CsvWriter {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("write_line", |lua, this, data: mlua::Value| match data {
            mlua::Value::Table(data) => {
                let time = data.get::<_, f64>("time")?;
                let output = data.get::<_, mlua::Value>("data")?;
                let csv_line = match output {
                    mlua::Value::Table(t) => {
                        let t = lua.from_value::<CoreOutput>(mlua::Value::Table(t))?;
                        t
                    }
                    _ => {
                        return Err(mlua::Error::RuntimeError("Invalid coreoutput".to_string()));
                    }
                };
                this.write_line(time, &csv_line)
            }
            _ => Err(mlua::Error::RuntimeError("Invalid data".to_string())),
        });
    }
}

fn new<'lua>(_lua: &'lua Lua, path: mlua::Value) -> LuaResult<CsvWriter> {
    match path {
        mlua::Value::String(path) => Ok(CsvWriter::new(path.to_str()?)?),
        _ => Err(mlua::Error::RuntimeError("Invalid path".to_string())),
    }
}

#[mlua::lua_module]
fn csv_viewer(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("new", lua.create_function(new)?)?;
    Ok(exports)
}
