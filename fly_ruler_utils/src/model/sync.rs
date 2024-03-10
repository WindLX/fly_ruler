use crate::{
    generated::core_output::{CoreOutput as CoreOutputGen, ViewMessage},
    plane_model::{Control, CoreOutput},
};
use async_trait::async_trait;
use mlua::LuaSerdeExt;
use prost::Message;
use std::sync::Arc;
use tokio::sync::{broadcast, watch, Mutex};

#[derive(Debug, Clone)]
pub enum Command {
    Control(Control),
    Extra(String),
    Exit,
}

impl mlua::UserData for Command {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("type", |_lua, this| {
            let typ = match this {
                Command::Control(_) => "Control",
                Command::Extra(_) => "Extra",
                Command::Exit => "Exit",
            };
            Ok(typ)
        });
        fields.add_field_method_get("value", |lua, this| match this {
            Command::Control(control) => lua.to_value(control),
            Command::Extra(extra) => Ok(mlua::Value::String(lua.create_string(extra).unwrap())),
            Command::Exit => Ok(mlua::Value::String(lua.create_string("Exit").unwrap())),
        })
    }
}

/// Create a state channel
/// this method is a wrapper of mpmc which means multi-producer, multi-consumer
pub fn state_channel(buffer: usize) -> (OutputSender, OutputReceiver) {
    let (sender, receiver) = broadcast::channel(buffer);
    let sender = OutputSender::new(Arc::new(Mutex::new(sender)));
    let receiver = OutputReceiver::new(Arc::new(Mutex::new(receiver)));
    (sender, receiver)
}

/// The sender end of state channel, which advised to be owned by plane model
#[derive(Clone)]
pub struct OutputSender(Arc<Mutex<broadcast::Sender<(f64, CoreOutput)>>>);

impl OutputSender {
    pub(crate) fn new(r: Arc<Mutex<broadcast::Sender<(f64, CoreOutput)>>>) -> Self {
        Self(r)
    }

    /// send plane state data to channel, waiting until there is capacity
    pub async fn send(&mut self, output: (f64, CoreOutput)) {
        let guard = self.0.lock().await;
        let _ = guard.send(output);
    }
}

/// The receiver end of state channel, which advised to be owned by controller or viewer
#[derive(Clone)]
pub struct OutputReceiver(Arc<Mutex<broadcast::Receiver<(f64, CoreOutput)>>>);

impl OutputReceiver {
    pub(crate) fn new(r: Arc<Mutex<broadcast::Receiver<(f64, CoreOutput)>>>) -> Self {
        Self(r)
    }

    /// Receives the next value for this receiver.
    /// This method returns `None` if the channel has been closed
    /// and there are no remaining messages in the channel's buffer.
    /// This indicates that no further values can ever be received from this Receiver.
    /// The channel is closed when all senders have been dropped.
    /// If there are no messages in the channel's buffer, but the channel has not yet been closed,
    /// this method will sleep until a message is sent or the channel is closed.
    pub async fn receive(&mut self) -> Option<(f64, CoreOutput)> {
        let mut guard = self.0.lock().await;
        guard.recv().await.ok()
    }

    /// Tries to receive the next value for this receiver.
    /// This method returns the [Empty] error if the channel is currently empty,
    /// but there are still outstanding [senders] or [permits].
    /// This method returns the [Disconnected] error if the channel is currently empty,
    /// and there are no outstanding [senders] or [permits].
    pub async fn try_receive(&mut self) -> Option<(f64, CoreOutput)> {
        let mut guard = self.0.lock().await;
        guard.try_recv().ok()
    }
}

impl mlua::UserData for OutputReceiver {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut("receive", |lua, this, ()| async move {
            let result = this.receive().await;
            match result {
                None => Ok(mlua::Nil),
                Some(value) => {
                    let (time, value) = value;
                    let table = lua.create_table()?;
                    table.set("time", time)?;
                    table.set("value", lua.create_any_userdata(value).unwrap())?;
                    Ok(mlua::Value::Table(table))
                }
            }
        });

        methods.add_async_method_mut("try_receive", |lua, this, ()| async move {
            let result = this.try_receive().await;
            match result {
                None => Ok(mlua::Nil),
                Some(value) => {
                    let (time, value) = value;
                    let table = lua.create_table()?;
                    table.set("time", time)?;
                    table.set("value", lua.create_any_userdata(value).unwrap())?;
                    Ok(mlua::Value::Table(table))
                }
            }
        });
    }
}

/// Create a command channel
/// A single-producer, multi-consumer channel that only retains the last sent value.
pub fn input_channel(init: Control) -> (InputSender, InputReceiver) {
    let (sender, receiver) = watch::channel::<Command>(Command::Control(init));
    let sender = InputSender::new(Arc::new(Mutex::new(sender)));
    let receiver = InputReceiver::new(Arc::new(Mutex::new(receiver)));
    (sender, receiver)
}

/// The sender end of command
#[derive(Clone)]
pub struct InputSender(Arc<Mutex<watch::Sender<Command>>>);

impl InputSender {
    pub fn new(r: Arc<Mutex<watch::Sender<Command>>>) -> Self {
        Self(r)
    }

    /// send data to channel and return None when channel is closed
    pub async fn send(&mut self, command: Command) -> Option<()> {
        let guard = self.0.lock().await;
        guard.send(command).ok()
    }
}

impl mlua::UserData for InputSender {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut("send", |_lua, this, command: mlua::Value| async move {
            let command = match command {
                mlua::Value::Nil => Command::Control(Control::default()),
                mlua::Value::UserData(ud) => {
                    if ud.is::<Command>() {
                        ud.take::<Command>().unwrap()
                    } else {
                        return Err(mlua::Error::RuntimeError("Invalid command".to_string()));
                    }
                }
                _ => {
                    return Err(mlua::Error::RuntimeError("Invalid command".to_string()));
                }
            };
            let _result = this.send(command).await;
            Ok(())
        });
    }
}

/// The receiver end of command channel
#[derive(Clone)]
pub struct InputReceiver(Arc<Mutex<watch::Receiver<Command>>>);

impl InputReceiver {
    pub fn new(r: Arc<Mutex<watch::Receiver<Command>>>) -> Self {
        Self(r)
    }

    /// Receive data, this method will block the thread until changed,
    /// if channel is closed, will return Exit Command
    pub async fn receive(&mut self) -> Command {
        let mut guard = self.0.lock().await;
        match guard.changed().await {
            Ok(_) => guard.borrow_and_update().clone(),
            Err(_) => Command::Exit,
        }
    }

    /// Try to receive command, if data is not update will get recent command
    /// Notice that attack command will be reset whether command update
    pub async fn try_receive(&mut self) -> Command {
        let guard = self.0.lock().await;
        let mut last = guard.borrow().clone();
        if let Command::Control(c) = last {
            last = Command::Control(c)
        }
        last
    }
}

pub trait AsInputer {
    fn get_receiver(&self) -> InputReceiver;
}

#[async_trait]
pub trait AsOutputer {
    fn set_receiver(&mut self, receiver: OutputReceiver);
    fn get_receiver(&self) -> Option<OutputReceiver>;

    async fn recv(&mut self) -> Option<(f64, CoreOutput)> {
        let receiver = self.get_receiver();
        match receiver {
            Some(mut receiver) => receiver.receive().await,
            None => None,
        }
    }
}

pub fn encode_view_message(time: f64, output: &CoreOutput) -> Vec<u8> {
    let msg = ViewMessage {
        time,
        output: Some(Into::<CoreOutputGen>::into(*output)),
    };
    msg.encode_to_vec()
}
