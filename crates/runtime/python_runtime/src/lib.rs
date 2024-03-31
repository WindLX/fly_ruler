use fly_ruler_utils::{plane_model::Control, InputSender, OutputReceiver};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use uuid::Uuid;

#[pyclass]
#[derive(Clone)]
pub struct OutputReceiverWrapper(OutputReceiver);

impl From<OutputReceiver> for OutputReceiverWrapper {
    fn from(value: OutputReceiver) -> Self {
        Self(value)
    }
}

#[pymethods]
impl OutputReceiverWrapper {
    async fn changed(&mut self) -> PyResult<()> {
        self.0
            .changed()
            .await
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    // fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
    //     methods.add_async_method_mut("changed", |_lua, this, ()| async move {});

    //     methods.add_method_mut("has_changed", |_lua, this, ()| {
    //         this.0.has_changed().map_err(mlua::Error::external)
    //     });

    //     methods.add_method("get", |lua, this, ()| {
    //         let (time, value) = this.0.get();
    //         let table = lua.create_table()?;
    //         table.set("time", time)?;
    //         table.set("data", lua.to_value(&value)?)?;
    //         Ok(mlua::Value::Table(table))
    //     });

    //     methods.add_method_mut("get_and_update", |lua, this, ()| {
    //         let (time, value) = this.0.get_and_update();
    //         let table = lua.create_table()?;
    //         table.set("time", time)?;
    //         table.set("data", lua.to_value(&value)?)?;
    //         Ok(mlua::Value::Table(table))
    //     });

    //     methods.add_method("clone", |_lua, this, ()| Ok(this.clone()));
    // }
}

#[pyclass]
pub struct InputSenderWrapper(InputSender);

impl From<InputSender> for InputSenderWrapper {
    fn from(value: InputSender) -> Self {
        Self(value)
    }
}

// impl mlua::UserData for InputSenderWrapper {
//     fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
//         methods.add_async_method(
//             "send",
//             |lua, this, control: Option<mlua::Table>| async move {
//                 match control {
//                     Some(control) => {
//                         let control = control.into_lua(lua)?;
//                         let control: Control = lua.from_value(control)?;
//                         this.0.send(&control).await.map_err(mlua::Error::external)
//                     }
//                     None => this
//                         .0
//                         .send(&Control::default())
//                         .await
//                         .map_err(mlua::Error::external),
//                 }
//             },
//         );
//     }
// }

#[pyclass]
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

#[pymethods]
impl UuidWrapper {
    #[staticmethod]
    pub fn new_v4() -> PyResult<Self> {
        Ok(Self(Uuid::new_v4()))
    }

    pub fn parse_str(s: &str) -> PyResult<Self> {
        Ok(Self(
            Uuid::parse_str(s).map_err(|e| PyRuntimeError::new_err(e.to_string()))?,
        ))
    }
}

// impl mlua::UserData for UuidWrapper {
//     fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
//         methods.add_meta_method("__tostring", |_: &'lua mlua::Lua, this, ()| {
//             Ok(this.inner().to_string())
//         });
//         methods.add_meta_method(
//             "__eq",
//             |_: &'lua mlua::Lua, this, other: mlua::UserDataRef<'lua, UuidWrapper>| {
//                 Ok(this.inner() == other.inner())
//             },
//         )
//     }
// }