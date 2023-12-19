use crate::plant_model::{Control, State};
use std::sync::Arc;
use tokio::sync::{mpsc, watch, Mutex};

#[derive(Debug, Clone)]
pub enum Command {
    Control(Control),
    Extra(String),
    Exit,
}

pub fn state_channel(buffer: usize) -> (StateSender, StateReceiver) {
    let (sender, receiver) = mpsc::channel(buffer);
    let sender = StateSender::new(Arc::new(Mutex::new(sender)));
    let receiver = StateReceiver::new(Arc::new(Mutex::new(receiver)));
    (sender, receiver)
}

#[derive(Debug, Clone)]
pub struct StateSender(Arc<Mutex<mpsc::Sender<State>>>);

impl StateSender {
    pub fn new(r: Arc<Mutex<mpsc::Sender<State>>>) -> Self {
        Self(r)
    }

    pub async fn send(&mut self, state: State) {
        let guard = self.0.lock().await;
        guard.send(state).await.unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct StateReceiver(Arc<Mutex<mpsc::Receiver<State>>>);

impl StateReceiver {
    pub fn new(r: Arc<Mutex<mpsc::Receiver<State>>>) -> Self {
        Self(r)
    }

    pub async fn receive(&mut self) -> State {
        let mut guard = self.0.lock().await;
        loop {
            if let Some(res) = guard.recv().await {
                return res;
            }
        }
    }
}

pub fn command_channel(init: Control) -> (CommandSender, CommandReceiver) {
    let (sender, receiver) = watch::channel::<Command>(Command::Control(init));
    let sender = CommandSender::new(Arc::new(Mutex::new(sender)));
    let receiver = CommandReceiver::new(Arc::new(Mutex::new(receiver)));
    (sender, receiver)
}

#[derive(Debug, Clone)]
pub struct CommandSender(Arc<Mutex<watch::Sender<Command>>>);

impl CommandSender {
    pub fn new(r: Arc<Mutex<watch::Sender<Command>>>) -> Self {
        Self(r)
    }

    pub async fn send(&mut self, command: Command) {
        let guard = self.0.lock().await;
        guard.send(command).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct CommandReceiver(Arc<Mutex<watch::Receiver<Command>>>);

impl CommandReceiver {
    pub fn new(r: Arc<Mutex<watch::Receiver<Command>>>) -> Self {
        Self(r)
    }

    pub async fn receive(&mut self) -> Command {
        let mut guard = self.0.lock().await;
        match guard.changed().await {
            Ok(_) => guard.borrow_and_update().clone(),
            Err(_) => Command::Exit,
        }
    }

    pub async fn try_receive(&mut self) -> Command {
        let mut guard = self.0.lock().await;
        if guard.has_changed().unwrap() {
            guard.borrow_and_update().clone()
        } else {
            Command::Control(Control::default())
        }
    }
}

pub trait IsController {
    fn get_receiver(&self) -> CommandReceiver;
}
