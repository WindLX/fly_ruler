pub mod manager;
pub mod system;

use env_logger::{fmt, Target, TimestampPrecision};
use lazy_static::lazy_static;
use log::{debug, error, info, trace, warn};
use lua_runtime::prelude::*;
use lua_runtime::UuidWrapper;
use system::SystemWrapper;

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
fn light_simulation_engine(lua: &Lua) -> LuaResult<LuaTable> {
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

    Ok(exports)
}
