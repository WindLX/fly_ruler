pub(crate) mod generated;
pub(crate) mod model;

use fly_ruler_utils::plane_model::CoreOutput;
use mlua::prelude::*;
use model::encode_view_message;

fn encode<'lua>(lua: &'lua Lua, view_msg: mlua::Table) -> LuaResult<mlua::String<'lua>> {
    let time = view_msg.get::<_, f64>("time")?;
    let message = view_msg.get::<_, mlua::Value>("view_msg")?;
    let proto = match message {
        mlua::Value::Table(t) => {
            let t = lua.from_value::<Vec<(u32, CoreOutput)>>(mlua::Value::Table(t))?;
            t
        }
        _ => {
            return Err(mlua::Error::RuntimeError("Invalid coreoutput".to_string()));
        }
    };
    let proto = encode_view_message(time, proto);
    lua.create_string(proto)
}

#[mlua::lua_module]
fn protobuf_viewer(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("encode", lua.create_function(encode)?)?;
    Ok(exports)
}
