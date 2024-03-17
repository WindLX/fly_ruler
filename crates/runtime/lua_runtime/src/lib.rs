use fly_ruler_utils::{plane_model::Control, Command, InputSender, OutputReceiver};
use mlua::LuaSerdeExt;
use uuid::Uuid;

#[derive(Clone)]
pub struct OutputReceiverWrapper(OutputReceiver);

impl From<OutputReceiver> for OutputReceiverWrapper {
    fn from(value: OutputReceiver) -> Self {
        Self(value)
    }
}

impl mlua::UserData for OutputReceiverWrapper {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut("changed", |_lua, this, ()| async move {
            this.0
                .changed()
                .await
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
        });

        methods.add_method_mut("has_changed", |_lua, this, ()| {
            this.0
                .has_changed()
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
        });

        methods.add_method("get", |lua, this, ()| {
            let (time, value) = this.0.get();
            let table = lua.create_table()?;
            table.set("time", time)?;
            table.set("data", lua.to_value(&value).unwrap())?;
            Ok(mlua::Value::Table(table))
        });

        methods.add_method_mut("get_and_update", |lua, this, ()| {
            let (time, value) = this.0.get_and_update();
            let table = lua.create_table()?;
            table.set("time", time)?;
            table.set("data", lua.to_value(&value).unwrap())?;
            Ok(mlua::Value::Table(table))
        });

        methods.add_method("clone", |_lua, this, ()| Ok(this.clone()));
    }
}

pub struct InputSenderWrapper(InputSender);

impl From<InputSender> for InputSenderWrapper {
    fn from(value: InputSender) -> Self {
        Self(value)
    }
}

pub fn table_to_cmd<'lua>(
    lua: &'lua mlua::Lua,
    command: mlua::Table<'lua>,
) -> mlua::Result<Command> {
    let mut cmd = Command::Control(Control::default());

    if command.contains_key("extra")? {
        cmd = Command::Extra(command.get("extra")?);
    }
    if command.contains_key("control")? {
        let ct: mlua::Table = command.get("control")?;
        cmd = Command::Control(lua.from_value(mlua::Value::Table(ct))?);
    }
    if command.contains_key("exit")? {
        cmd = Command::Exit;
    }
    Ok(cmd)
}

pub fn cmd_to_table<'lua>(
    lua: &'lua mlua::Lua,
    command: Command,
) -> mlua::Result<mlua::Table<'lua>> {
    let cmd = lua.create_table()?;
    match command {
        Command::Control(ct) => {
            let ct = lua.to_value(&ct)?;
            cmd.set("control", ct)?;
        }
        Command::Exit => {
            cmd.set("exit", true)?;
        }
        Command::Extra(extra) => {
            let extra = lua.to_value(&extra)?;
            cmd.set("extra", extra)?;
        }
    }
    Ok(cmd)
}

impl mlua::UserData for InputSenderWrapper {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method(
            "send",
            |lua, this, command: Option<mlua::Table>| async move {
                match command {
                    Some(command) => {
                        let cmd = table_to_cmd(lua, command)?;
                        this.0
                            .send(&cmd)
                            .await
                            .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
                    }
                    None => this
                        .0
                        .send(&Command::Control(Control::default()))
                        .await
                        .map_err(|e| mlua::Error::RuntimeError(e.to_string())),
                }
            },
        );
    }
}

#[derive(Clone)]
pub struct UuidWrapper(Uuid);

impl PartialEq<UuidWrapper> for UuidWrapper {
    fn eq(&self, other: &UuidWrapper) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for UuidWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for UuidWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Eq for UuidWrapper {}

impl From<Uuid> for UuidWrapper {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl UuidWrapper {
    pub fn inner(&self) -> Uuid {
        self.0.clone()
    }
}

impl mlua::UserData for UuidWrapper {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method("__tostring", |_: &'lua mlua::Lua, this, ()| {
            Ok(this.inner().to_string())
        });
        methods.add_meta_method(
            "__eq",
            |_: &'lua mlua::Lua, this, other: mlua::UserDataRef<'lua, UuidWrapper>| {
                Ok(this.inner() == other.inner())
            },
        )
    }
}
