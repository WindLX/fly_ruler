use fly_ruler_utils::{
    command_channel, plant_model::Control, Command, CommandReceiver, CommandSender, IsController,
};
use log::{info, trace};
use std::sync::Arc;
use stick::{Event, Listener};
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct StickController {
    id: usize,
    stick: Option<stick::Controller>,
    sender: CommandSender,
    receiver: CommandReceiver,
}

impl StickController {
    pub async fn new(id: usize, init: Control) -> Self {
        let lis = Listener::default();
        let controller = lis.await;
        // let stick = Arc::new(Mutex::new(Some(controller)));
        let stick = Some(controller);
        let (tx, rx) = command_channel(init);
        Self {
            id,
            stick,
            sender: tx,
            receiver: rx,
        }
    }

    pub async fn send(&mut self) -> Command {
        // let mut c = self.stick.lock().await;
        let mut events = vec![0.0, 0.0, 0.0, 0.0];
        for i in 0..4 {
            if let Some(c) = &mut self.stick {
                match c.await {
                    Event::Exit(_) | Event::Disconnect => {
                        info!("{} exit", self.id);
                        self.sender.send(Command::Exit).await;
                        return Command::Exit;
                    }
                    e => match e {
                        Event::TriggerL(thrust) => events[0] = thrust * 18000.0 * 0.5 + 1000.0,
                        Event::JoyX(rudder) => events[3] = rudder * 30.0 * 0.5,
                        Event::JoyY(aileron) => events[2] = aileron * 21.5 * 0.5,
                        Event::CamX(elevator) => events[1] = elevator * 25.0 * 0.5,
                        _ => {}
                    },
                }
            } else {
                return Command::Exit;
            };
        }

        let command = Command::Control(Control::from(events));
        self.sender.send(command.clone()).await;
        command
    }
}

impl IsController for StickController {
    fn get_receiver(&self) -> CommandReceiver {
        self.receiver.clone()
    }
}
