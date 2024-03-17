use crate::{
    error::FrError,
    plane_model::{Control, CoreOutput},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, watch};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Control(Control),
    Extra(String),
    Exit,
}

impl Default for Command {
    fn default() -> Self {
        Command::Control(Control::default())
    }
}

/// Create a state channel
/// this method is a wrapper of spmc which means single-producer, multi-consumer
pub fn state_channel(init: &CoreOutput) -> (OutputSender, OutputReceiver) {
    let (sender, receiver) = watch::channel::<(f64, CoreOutput)>((0.0, *init));
    let sender = OutputSender::new(sender);
    let receiver = OutputReceiver::new(receiver);
    (sender, receiver)
}

/// The sender end of state channel, which advised to be owned by plane model
#[derive(Clone)]
pub struct OutputSender(Arc<watch::Sender<(f64, CoreOutput)>>);

impl OutputSender {
    pub(crate) fn new(r: watch::Sender<(f64, CoreOutput)>) -> Self {
        Self(Arc::new(r))
    }

    pub fn send(&self, output: &(f64, CoreOutput)) -> Result<(), FrError> {
        let sender = &self.0;
        Ok(sender
            .send(*output)
            .map_err(|e| FrError::Sync(e.to_string()))?)
    }

    pub fn send_replace(&self, output: &(f64, CoreOutput)) -> (f64, CoreOutput) {
        let sender = &self.0;
        sender.send_replace(*output)
    }

    pub fn subscribe(&self) -> OutputReceiver {
        let sender = &self.0;
        OutputReceiver(sender.subscribe())
    }
}

#[derive(Clone)]
pub struct OutputReceiver(watch::Receiver<(f64, CoreOutput)>);

impl OutputReceiver {
    pub(crate) fn new(r: watch::Receiver<(f64, CoreOutput)>) -> Self {
        Self(r)
    }

    pub async fn changed(&mut self) -> Result<(), FrError> {
        let recv = &mut self.0;
        recv.changed()
            .await
            .map_err(|e| FrError::Sync(e.to_string()))
    }

    pub fn has_changed(&self) -> Result<bool, FrError> {
        let recv = &self.0;
        recv.has_changed().map_err(|e| FrError::Sync(e.to_string()))
    }

    pub fn get(&self) -> (f64, CoreOutput) {
        let recv = &self.0;
        recv.borrow().clone()
    }

    pub fn get_and_update(&mut self) -> (f64, CoreOutput) {
        let recv = &mut self.0;
        recv.borrow_and_update().clone()
    }
}

/// Create a command channel
/// A multi-producer, single-consumer channel that only retains the last sent value.
pub fn input_channel(buffer: usize) -> (InputSender, InputReceiver) {
    let (sender, receiver) = mpsc::channel::<Command>(buffer);
    let sender = InputSender::new(sender);
    let receiver = InputReceiver::new(receiver);
    (sender, receiver)
}

/// The sender end of command
#[derive(Clone)]
pub struct InputSender(mpsc::Sender<Command>);

impl InputSender {
    pub fn new(r: mpsc::Sender<Command>) -> Self {
        Self(r)
    }

    pub async fn send(&self, command: &Command) -> Result<(), FrError> {
        let sender = &self.0;
        Ok(sender
            .send(command.clone())
            .await
            .map_err(|e| FrError::Sync(e.to_string()))?)
    }
}

/// The receiver end of command channel
pub struct InputReceiver(mpsc::Receiver<Command>);

impl InputReceiver {
    pub fn new(r: mpsc::Receiver<Command>) -> Self {
        Self(r)
    }

    pub async fn recv(&mut self) -> Option<Command> {
        self.0.recv().await
    }
}
