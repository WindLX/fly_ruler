pub mod manager;
pub mod system;

use crate::system::System;
use env_logger::{fmt, Target, TimestampPrecision};
use fly_ruler_utils::plane_model::Control;
use fly_ruler_utils::Command;
use lazy_static::lazy_static;
use log::{debug, error, info, trace, warn};
use lua_runtime::CommandWrapper;
use mlua::prelude::*;

lazy_static! {
    static ref RT: tokio::runtime::Runtime = {
        std::thread::spawn(|| RT.block_on(futures::future::pending::<()>()));
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    };
    static ref GUARD: tokio::runtime::EnterGuard<'static> = RT.enter();
}

#[mlua::lua_module]
fn lua_system(lua: &Lua) -> LuaResult<LuaTable> {
    let _guard = &*GUARD;
    let exports = lua.create_table()?;

    // system
    let system = lua.create_table()?;
    system.set("new", LuaFunction::wrap(|_: &Lua, ()| Ok(System::new())))?;

    // command
    let command = lua.create_table()?;
    command.set(
        "control",
        LuaFunction::wrap(|lua: &Lua, value: mlua::Value| match value {
            mlua::Value::Table(_) => Ok(CommandWrapper::from(Command::Control(
                lua.from_value(value)?,
            ))),
            mlua::Value::Nil => Ok(CommandWrapper::from(Command::Control(Control::default()))),
            _ => Err(mlua::Error::RuntimeError("Invalid command".to_string())),
        }),
    )?;
    command.set("exit", CommandWrapper::from(Command::Exit))?;
    command.set(
        "extra",
        CommandWrapper::from(Command::Extra("".to_string())),
    )?;
    command.set(
        "default",
        CommandWrapper::from(Command::Control(Control::default())),
    )?;

    let logger = lua.create_table()?;
    logger.set(
        "init",
        LuaFunction::wrap(|_lua: &Lua, value: Option<mlua::Table>| {
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
    )?;

    logger.set(
        "trace",
        LuaFunction::wrap(|_: &Lua, msg: mlua::String| {
            trace!("{}", msg.to_str()?);
            Ok(())
        }),
    )?;
    logger.set(
        "debug",
        LuaFunction::wrap(|_: &Lua, msg: mlua::String| {
            debug!("{}", msg.to_str()?);
            Ok(())
        }),
    )?;
    logger.set(
        "info",
        LuaFunction::wrap(|_: &Lua, msg: mlua::String| {
            info!("{}", msg.to_str()?);
            Ok(())
        }),
    )?;
    logger.set(
        "warn",
        LuaFunction::wrap(|_: &Lua, msg: mlua::String| {
            warn!("{}", msg.to_str()?);
            Ok(())
        }),
    )?;
    logger.set(
        "error",
        LuaFunction::wrap(|_: &Lua, msg: mlua::String| {
            error!("{}", msg.to_str()?);
            Ok(())
        }),
    )?;

    exports.set("system", system)?;
    exports.set("command", command)?;
    exports.set("logger", logger)?;

    Ok(exports)
}
