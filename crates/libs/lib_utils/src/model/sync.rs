use crate::plane_model::{Control, CoreOutput};
use std::sync::Arc;
use tokio::sync::{broadcast, watch, Mutex};

#[derive(Debug, Clone)]
pub enum Command {
    Control(Control),
    Extra(String),
    Exit,
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