use env_logger::{fmt, Target, TimestampPrecision};
use fly_ruler_utils::plane_model::Control;
use fly_ruler_utils::Command;
use log::{debug, error, info, trace, warn};
use lua_runtime::CommandWrapper;
use lua_system::system::System;
use mlua::{Function, Lua, LuaSerdeExt};
use std::env;
use std::path::Path;

fn main() {
    let main_lua = env::args().nth(1).unwrap_or_else(|| "main.lua".to_string());

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move {
        let lua = unsafe { Lua::unsafe_new() };
        let module = lua.create_table().unwrap();

        // system
        let system = lua.create_table().unwrap();
        system
            .set("new", Function::wrap(|_: &Lua, ()| Ok(System::new())))
            .unwrap();

        // command
        let command = lua.create_table().unwrap();
        command
            .set(
                "control",
                Function::wrap(|lua: &Lua, value: mlua::Value| match value {
                    mlua::Value::Table(_) => Ok(CommandWrapper::from(Command::Control(
                        lua.from_value(value)?,
                    ))),
                    mlua::Value::Nil => {
                        Ok(CommandWrapper::from(Command::Control(Control::default())))
                    }
                    _ => Err(mlua::Error::RuntimeError("Invalid command".to_string())),
                }),
            )
            .unwrap();
        command
            .set("exit", CommandWrapper::from(Command::Exit))
            .unwrap();
        command
            .set(
                "extra",
                CommandWrapper::from(Command::Extra("".to_string())),
            )
            .unwrap();
        command
            .set(
                "default",
                CommandWrapper::from(Command::Control(Control::default())),
            )
            .unwrap();

        let logger = lua.create_table().unwrap();
        logger
            .set(
                "init",
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

        logger
            .set(
                "trace",
                Function::wrap(|_: &Lua, msg: mlua::String| {
                    trace!("{}", msg.to_str().unwrap());
                    Ok(())
                }),
            )
            .unwrap();
        logger
            .set(
                "debug",
                Function::wrap(|_: &Lua, msg: mlua::String| {
                    debug!("{}", msg.to_str().unwrap());
                    Ok(())
                }),
            )
            .unwrap();
        logger
            .set(
                "info",
                Function::wrap(|_: &Lua, msg: mlua::String| {
                    info!("{}", msg.to_str().unwrap());
                    Ok(())
                }),
            )
            .unwrap();
        logger
            .set(
                "warn",
                Function::wrap(|_: &Lua, msg: mlua::String| {
                    warn!("{}", msg.to_str().unwrap());
                    Ok(())
                }),
            )
            .unwrap();
        logger
            .set(
                "error",
                Function::wrap(|_: &Lua, msg: mlua::String| {
                    error!("{}", msg.to_str().unwrap());
                    Ok(())
                }),
            )
            .unwrap();

        module.set("system", system).unwrap();
        module.set("command", command).unwrap();
        module.set("logger", logger).unwrap();

        lua.globals().set("fly_ruler", module).unwrap();
        let res = lua.load(Path::new(&main_lua)).exec_async().await;
        match res {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    });
}
