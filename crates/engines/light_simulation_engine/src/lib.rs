pub mod manager;
pub mod system;

use lazy_static::lazy_static;
use lua_runtime::prelude::*;
use lua_runtime::UuidWrapper;
use system::SystemWrapper;
use tracing::event;
use tracing::span;
use tracing::Level;
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};

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

async fn sleep(_lua: &Lua, duration: LuaNumber) -> LuaResult<()> {
    let duration = duration as u64;
    tokio::time::sleep(std::time::Duration::from_millis(duration)).await;
    Ok(())
}

fn to_radians(_lua: &Lua, angle: LuaNumber) -> LuaResult<f64> {
    let angle = angle as f64;
    Ok(angle.to_radians())
}

fn to_degrees(_lua: &Lua, angle: LuaNumber) -> LuaResult<f64> {
    let angle = angle as f64;
    Ok(angle.to_degrees())
}

fn light_simulation_engine_constructor(lua: &Lua) -> LuaResult<LuaTable> {
    let _guard = &*GUARD;
    let exports = lua.create_table()?;

    // system
    let system = lua.create_table()?;
    system.set(
        "new",
        LuaFunction::wrap(|_: &Lua, ()| Ok(SystemWrapper::new())),
    )?;

    let logger = lua.create_table()?;
    logger.set(
        "init",
        LuaFunction::wrap(|_lua: &Lua, value: Option<mlua::Table>| {
            let log_filter: String;
            let log_dir: Option<String>;
            let log_file: Option<String>;
            log_filter = match value {
                Some(ref value) => {
                    let filter: Option<String> = value.get("filter")?;
                    match filter {
                        Some(filter) => filter,
                        _ => String::new(),
                    }
                }
                None => String::new(),
            };
            log_dir = match value {
                Some(ref value) => {
                    let dir: Option<String> = value.get("dir")?;
                    match dir {
                        Some(dir) => Some(dir),
                        _ => None,
                    }
                }
                None => None,
            };
            log_file = match value {
                Some(ref value) => {
                    let file: Option<String> = value.get("file")?;
                    match file {
                        Some(file) => Some(file),
                        _ => None,
                    }
                }
                None => None,
            };

            let env_filter = EnvFilter::new(log_filter);
            let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);

            match (log_dir, log_file) {
                (Some(log_dir), Some(log_file)) => {
                    let file_appender = rolling::never(log_dir, log_file);
                    let (non_blocking_appender, _guard) = non_blocking(file_appender);
                    let file_layer = fmt::layer()
                        .with_ansi(false)
                        .with_writer(non_blocking_appender);
                    Registry::default()
                        .with(env_filter)
                        .with(ErrorLayer::default())
                        .with(formatting_layer)
                        .with(file_layer)
                        .init();
                }
                _ => {
                    Registry::default()
                        .with(env_filter)
                        .with(ErrorLayer::default())
                        .with(formatting_layer)
                        .init();
                }
            }

            Ok(())
        }),
    )?;

    logger.set(
        "trace",
        LuaFunction::wrap(|_: &Lua, msg: mlua::String| {
            let s = span!(Level::TRACE, "lua");
            let _enter = s.enter();
            event!(Level::TRACE, "{}", msg.to_str()?);
            Ok(())
        }),
    )?;
    logger.set(
        "debug",
        LuaFunction::wrap(|_: &Lua, msg: mlua::String| {
            let s = span!(Level::DEBUG, "lua");
            let _enter = s.enter();
            event!(Level::DEBUG, "{}", msg.to_str()?);
            Ok(())
        }),
    )?;
    logger.set(
        "info",
        LuaFunction::wrap(|_: &Lua, msg: mlua::String| {
            let s = span!(Level::INFO, "lua");
            let _enter = s.enter();
            event!(Level::INFO, "{}", msg.to_str()?);
            Ok(())
        }),
    )?;
    logger.set(
        "warn",
        LuaFunction::wrap(|_: &Lua, msg: mlua::String| {
            let s = span!(Level::WARN, "lua");
            let _enter = s.enter();
            event!(Level::WARN, "{}", msg.to_str()?);
            Ok(())
        }),
    )?;
    logger.set(
        "error",
        LuaFunction::wrap(|_: &Lua, msg: mlua::String| {
            let s = span!(Level::ERROR, "lua");
            let _enter = s.enter();
            event!(Level::ERROR, "{}", msg.to_str()?);
            Ok(())
        }),
    )?;

    let uuid = lua.create_table()?;
    uuid.set(
        "new_v4",
        LuaFunction::wrap(|_: &Lua, ()| {
            let u = UuidWrapper::new_v4();
            Ok(UuidWrapper::from(u))
        }),
    )?;
    uuid.set(
        "parse_str",
        LuaFunction::wrap(|_: &Lua, uuid: LuaString| {
            let u = UuidWrapper::parse_str(uuid.to_str()?)?;
            Ok(UuidWrapper::from(u))
        }),
    )?;

    exports.set("system", system)?;
    exports.set("logger", logger)?;
    exports.set("uuid", uuid)?;
    exports.set("sleep", lua.create_async_function(sleep)?)?;
    exports.set("to_radians", lua.create_function(to_radians)?)?;
    exports.set("to_degrees", lua.create_function(to_degrees)?)?;

    Ok(exports)
}

#[cfg(target_os = "windows")]
#[mlua::lua_module]
fn light_simulation_engine(lua: &Lua) -> LuaResult<LuaTable> {
    light_simulation_engine_constructor(lua)
}

#[cfg(target_os = "linux")]
#[mlua::lua_module]
fn liblight_simulation_engine(lua: &Lua) -> LuaResult<LuaTable> {
    light_simulation_engine_constructor(lua)
}
