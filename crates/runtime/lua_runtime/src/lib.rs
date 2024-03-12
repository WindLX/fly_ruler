use fly_ruler_utils::{plane_model::Control, Command, InputSender, OutputReceiver};
use mlua::LuaSerdeExt;

#[derive(Debug, Clone)]
pub struct CommandWrapper(Command);

impl From<Command> for CommandWrapper {
    fn from(value: Command) -> Self {
        Self(value)
    }
}

impl CommandWrapper {
    pub fn inner(&self) -> Command {
        self.0.clone()
    }
}

impl mlua::UserData for CommandWrapper {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("type", |_lua, this| {
            let typ = match this.0 {
                Command::Control(_) => "Control",
                Command::Extra(_) => "Extra",
                Command::Exit => "Exit",
            };
            Ok(typ)
        });
        fields.add_field_method_get("value", |lua, this| match &this.0 {
            Command::Control(control) => lua.to_value(&control),
            Command::Extra(extra) => Ok(mlua::Value::String(lua.create_string(extra).unwrap())),
            Command::Exit => Ok(mlua::Value::String(lua.create_string("Exit").unwrap())),
        })
    }
}

pub struct OutputReceiverWrapper(OutputReceiver);

impl From<OutputReceiver> for OutputReceiverWrapper {
    fn from(value: OutputReceiver) -> Self {
        Self(value)
    }
}

impl mlua::UserData for OutputReceiverWrapper {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut("receive", |lua, this, ()| async move {
            let result = this.0.receive().await;
            match result {
                None => Ok(mlua::Nil),
                Some(value) => {
                    let (time, value) = value;
                    let table = lua.create_table()?;
                    table.set("time", time)?;
                    table.set("data", lua.to_value(&value).unwrap())?;
                    Ok(mlua::Value::Table(table))
                }
            }
        });

        methods.add_async_method_mut("try_receive", |lua, this, ()| async move {
            let result = this.0.try_receive().await;
            match result {
                None => Ok(mlua::Nil),
                Some(value) => {
                    let (time, value) = value;
                    let table = lua.create_table()?;
                    table.set("time", time)?;
                    table.set("data", lua.to_value(&value).unwrap())?;
                    Ok(mlua::Value::Table(table))
                }
            }
        });
    }
}

pub struct InputSenderWrapper(InputSender);

impl From<InputSender> for InputSenderWrapper {
    fn from(value: InputSender) -> Self {
        Self(value)
    }
}

impl mlua::UserData for InputSenderWrapper {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut("send", |_lua, this, command: mlua::Value| async move {
            let command = match command {
                mlua::Value::Nil => Command::Control(Control::default()),
                mlua::Value::UserData(ud) => {
                    if ud.is::<CommandWrapper>() {
                        ud.take::<CommandWrapper>().unwrap().inner()
                    } else {
                        return Err(mlua::Error::RuntimeError("Invalid command".to_string()));
                    }
                }
                _ => {
                    return Err(mlua::Error::RuntimeError("Invalid command".to_string()));
                }
            };
            let _result = this.0.send(command).await;
            Ok(())
        });
    }
}
