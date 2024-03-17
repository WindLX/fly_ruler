pub mod manager;
pub mod system;

use env_logger::{fmt, Target, TimestampPrecision};
use lazy_static::lazy_static;
use log::{debug, error, info, trace, warn};
use lua_runtime::UuidWrapper;
use mlua::prelude::*;
use std::collections::BTreeSet;
use system::SystemWrapper;
use uuid::Uuid;

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

struct UuidSet(BTreeSet<UuidWrapper>);

impl UuidSet {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn add(&mut self, uuid: UuidWrapper) {
        self.0.insert(uuid);
    }

    pub fn remove(&mut self, uuid: &UuidWrapper) {
        self.0.remove(uuid);
    }

    pub fn contains(&self, uuid: &UuidWrapper) -> bool {
        self.0.contains(uuid)
    }
}

impl LuaUserData for UuidSet {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("add", |_, this, uuid: LuaUserDataRef<UuidWrapper>| {
            this.add(uuid.clone());
            Ok(())
        });
        methods.add_method_mut("remove", |_, this, uuid: LuaUserDataRef<UuidWrapper>| {
            this.remove(&*uuid);
            Ok(())
        });
        methods.add_method("contains", |_, this, uuid: LuaUserDataRef<UuidWrapper>| {
            Ok(this.contains(&uuid))
        });
        methods.add_method("to_table", |lua, this, ()| {
            let t = lua.create_table()?;
            for uuid in this.0.iter() {
                t.raw_push(uuid.clone())?;
            }
            Ok(t)
        });
        methods.add_meta_function("__len", |_, this: LuaUserDataRef<UuidSet>| {
            Ok(this.0.len() as u32)
        });
    }
}

#[mlua::lua_module]
fn lua_system(lua: &Lua) -> LuaResult<LuaTable> {
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

    let uuid_set = lua.create_table()?;
    uuid_set.set("new", LuaFunction::wrap(|_: &Lua, ()| Ok(UuidSet::new())))?;
    uuid_set.set(
        "uuid_v4",
        LuaFunction::wrap(|_: &Lua, uuid: mlua::String| {
            let u = Uuid::parse_str(uuid.to_str()?)
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;
            Ok(UuidWrapper::from(u))
        }),
    )?;

    exports.set("system", system)?;
    exports.set("logger", logger)?;
    exports.set("uuid_set", uuid_set)?;

    Ok(exports)
}
