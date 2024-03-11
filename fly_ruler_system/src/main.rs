use env_logger::{fmt, Target, TimestampPrecision};
use fly_ruler_system::system::System;
use fly_ruler_utils::Command;
use log::{debug, error, info, trace, warn};
use mlua::{Function, Lua, LuaSerdeExt};
use std::path::Path;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move {
        let lua = Lua::new();
        let module = lua.create_table().unwrap();
        module
            .set(
                "create_system",
                Function::wrap(|_: &Lua, ()| Ok(System::new())),
            )
            .unwrap();
        module
            .set(
                "create_command",
                Function::wrap(|lua: &Lua, value: mlua::Value| match value {
                    mlua::Value::String(s) => {
                        if s == "Exit" {
                            Ok(Command::Exit)
                        } else {
                            Ok(Command::Extra(s.to_string_lossy().to_string()))
                        }
                    }
                    mlua::Value::Table(_) => Ok(Command::Control(lua.from_value(value)?)),
                    _ => Err(mlua::Error::RuntimeError("Invalid command".to_string())),
                }),
            )
            .unwrap();
        module
            .set(
                "init_logger",
                Function::wrap(|_lua: &Lua, value: Option<mlua::Table>| {
                    let target: Target;
                    let timestamp: Option<fmt::TimestampPrecision>;
                    target = match value {
                        Some(ref value) => {
                            let target: Option<String> = value.get("target")?;
                            match target {
                                Some(target) => match target.as_str() {
                                    "Stdout" => Target::Stdout,
                                    _ => Target::Stderr,
                                },
                                _ => Target::Stderr,
                            }
                        }
                        None => Target::Stderr,
                    };
                    timestamp = match value {
                        Some(value) => {
                            let timestamp: Option<String> = value.get("timestamp")?;
                            match timestamp {
                                Some(timestamp) => {
                                    let ts = match timestamp.as_str() {
                                        "Millis" => TimestampPrecision::Millis,
                                        "Nanos" => TimestampPrecision::Nanos,
                                        "Seconds" => TimestampPrecision::Seconds,
                                        _ => TimestampPrecision::Micros,
                                    };
                                    Some(ts)
                                }
                                _ => None,
                            }
                        }
                        None => None,
                    };
                    env_logger::builder()
                        .target(target)
                        .format_timestamp(timestamp)
                        .init();
                    Ok(())
                }),
            )
            .unwrap();
        module
            .set(
                "trace",
                Function::wrap(|_: &Lua, msg: mlua::String| {
                    trace!("{}", msg.to_str().unwrap());
                    Ok(())
                }),
            )
            .unwrap();
        module
            .set(
                "debug",
                Function::wrap(|_: &Lua, msg: mlua::String| {
                    debug!("{}", msg.to_str().unwrap());
                    Ok(())
                }),
            )
            .unwrap();
        module
            .set(
                "info",
                Function::wrap(|_: &Lua, msg: mlua::String| {
                    info!("{}", msg.to_str().unwrap());
                    Ok(())
                }),
            )
            .unwrap();
        module
            .set(
                "warn",
                Function::wrap(|_: &Lua, msg: mlua::String| {
                    warn!("{}", msg.to_str().unwrap());
                    Ok(())
                }),
            )
            .unwrap();
        module
            .set(
                "error",
                Function::wrap(|_: &Lua, msg: mlua::String| {
                    error!("{}", msg.to_str().unwrap());
                    Ok(())
                }),
            )
            .unwrap();

        lua.globals().set("fly_ruler", module).unwrap();
        let _res = lua.load(Path::new("./main.lua")).exec_async().await;
    });
}
