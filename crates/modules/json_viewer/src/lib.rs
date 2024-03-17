use fly_ruler_utils::{plane_model::CoreOutput, Command};
use lua_runtime::cmd_to_table;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ViewMessage {
    id: String,
    output: Option<CoreOutput>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ViewMessageGroup {
    time: f64,
    view_msg: Vec<ViewMessage>,
}

fn encode_view_message(time: f64, msg: Vec<(String, CoreOutput)>) -> LuaResult<String> {
    let msg_group = ViewMessageGroup {
        time,
        view_msg: msg
            .into_iter()
            .map(|(id, output)| ViewMessage {
                id,
                output: Some(Into::<CoreOutput>::into(output)),
            })
            .collect(),
    };
    let json_str = serde_json::to_string(&msg_group);
    match json_str {
        Ok(js) => Ok(js),
        Err(e) => Err(mlua::Error::RuntimeError(format!(
            "Failed to serialize ViewMessage: {}",
            e
        ))),
    }
}

fn encode<'lua>(lua: &'lua Lua, view_msg: mlua::Value) -> LuaResult<mlua::String<'lua>> {
    match view_msg {
        mlua::Value::Table(view_msg) => {
            let time = view_msg.get::<_, f64>("time")?;
            let message = view_msg.get::<_, mlua::Value>("view_msg")?;
            let json = match message {
                mlua::Value::Table(t) => {
                    let t = lua.from_value::<Vec<(String, CoreOutput)>>(mlua::Value::Table(t))?;
                    t
                }
                _ => {
                    return Err(mlua::Error::RuntimeError("Invalid view_msg".to_string()));
                }
            };
            let json = encode_view_message(time, json)?;
            lua.create_string(json)
        }
        _ => Err(mlua::Error::RuntimeError("Invalid view_msg".to_string())),
    }
}

fn decode<'lua>(lua: &'lua Lua, command: mlua::String<'lua>) -> LuaResult<LuaTable<'lua>> {
    let command: Command = serde_json::from_str(command.to_str()?)
        .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;
    cmd_to_table(lua, command)
}

#[mlua::lua_module]
fn json_viewer(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("encode", lua.create_function(encode)?)?;
    exports.set("decode_cmd", lua.create_function(decode)?)?;
    Ok(exports)
}
