use crate::{
    generated::core_output::{CoreOutput as CoreOutputGen, ViewMessage},
    plane_model::{Control, CoreOutput},
};
use prost::Message;
use std::sync::Arc;
use tokio::sync::{broadcast, watch, Mutex};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub enum Command {
    Control(Control, isize),
    Extra(String),
    Exit,
}

/// Create a state channel
/// this method is a wrapper of mpsc which means multi-producer, single-consumer
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
    /// This method returns `None`` if the channel has been closed
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

/// Create a command channel
/// A single-producer, multi-consumer channel that only retains the last sent value.
pub fn command_channel(init: Control) -> (CommandSender, CommandReceiver) {
    let (sender, receiver) = watch::channel::<Command>(Command::Control(init, -1));
    let sender = CommandSender::new(Arc::new(Mutex::new(sender)));
    let receiver = CommandReceiver::new(Arc::new(Mutex::new(receiver)));
    (sender, receiver)
}

/// The sender end of command
#[derive(Clone)]
pub struct CommandSender(Arc<Mutex<watch::Sender<Command>>>);

impl CommandSender {
    pub fn new(r: Arc<Mutex<watch::Sender<Command>>>) -> Self {
        Self(r)
    }

    /// send data to channel and return None when channel is closed
    pub async fn send(&mut self, command: Command) -> Option<()> {
        let guard = self.0.lock().await;
        guard.send(command).ok()
    }
}

/// The receiver end of command channel
#[derive(Clone)]
pub struct CommandReceiver(Arc<Mutex<watch::Receiver<Command>>>);

impl CommandReceiver {
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
        if let Command::Control(c, _t) = last {
            last = Command::Control(c, -1)
        }
        last
    }
}

pub trait IsController {
    fn get_receiver(&self) -> CommandReceiver;
}

pub struct ViewerCancellationToken(Arc<CancellationToken>);

impl ViewerCancellationToken {
    pub fn new() -> Self {
        Self(Arc::new(CancellationToken::new()))
    }

    pub fn cancel(&self) {
        self.0.cancel();
    }

    pub fn is_cancelled(&self) -> bool {
        self.0.is_cancelled()
    }
}

impl Clone for ViewerCancellationToken {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub trait IsViewer {
    fn set_receiver(&mut self, receiver: OutputReceiver);
}

pub fn encode_view_message(time: f64, output: &CoreOutput) -> Vec<u8> {
    let msg = ViewMessage {
        time,
        output: Some(Into::<CoreOutputGen>::into(*output)),
    };
    msg.encode_to_vec()
}
