use fly_ruler_utils::plane_model::CoreOutput;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ViewMessage {
    time: f64,
    output: Option<CoreOutput>,
}

fn encode_view_message(time: f64, output: &CoreOutput) -> LuaResult<String> {
    let msg = ViewMessage {
        time,
        output: Some(Into::<CoreOutput>::into(*output)),
    };
    let json_str = serde_json::to_string(&msg);
    match json_str {
        Ok(js) => Ok(js),
        Err(e) => Err(mlua::Error::RuntimeError(format!(
            "Failed to serialize ViewMessage: {}",
            e
        ))),
    }
}

fn encode<'lua>(lua: &'lua Lua, view_msg: mlua::Table) -> LuaResult<mlua::String<'lua>> {
    let time = view_msg.get::<_, f64>("time")?;
    let message = view_msg.get::<_, mlua::Value>("data")?;
    let json = match message {
        mlua::Value::Table(t) => {
            let t = lua.from_value::<CoreOutput>(mlua::Value::Table(t))?;
            t
        }
        _ => {
            return Err(mlua::Error::RuntimeError("Invalid coreoutput".to_string()));
        }
    };
    let json = encode_view_message(time, &json)?;
    lua.create_string(json)
}

#[mlua::lua_module]
fn json_viewer(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("encode", lua.create_function(encode)?)?;
    Ok(exports)
}
