use fly_ruler_utils::{
    input_channel, plane_model::Control, AsInputer, Command, InputReceiver, InputSender,
};
use log::{info, trace};
use std::thread::JoinHandle;
use stick::{Event, Listener};

pub struct StickController {
    id: usize,
    stick: Option<stick::Controller>,
    sender: InputSender,
    receiver: InputReceiver,
}

impl StickController {
    pub async fn new(id: usize, init: Control) -> Self {
        let lis = Listener::default();
        let controller = lis.await;
        info!("stick {id} connected");
        let stick = Some(controller);
        let (tx, rx) = input_channel(init);
        Self {
            id,
            stick,
            sender: tx,
            receiver: rx,
        }
    }

    async fn send(&mut self) -> Command {
        let mut events = vec![0.0, 0.0, 0.0, 0.0];
        for _ in 0..4 {
            if let Some(c) = &mut self.stick {
                let e = c.await;
                trace!("event: {e}");
                match e {
                    Event::Exit(true) | Event::Disconnect | Event::ActionB(true) => {
                        info!("{} exit", self.id);
                        self.sender.send(Command::Exit).await;
                        return Command::Exit;
                    }
                    e => match e {
                        #[cfg(target_family = "windows")]
                        Event::TriggerL(thrust) => events[0] = thrust * 18000.0 + 1000.0,
                        #[cfg(target_family = "unix")]
                        Event::JoyZ(thrust) => events[0] = thrust / 0.03 * 18000.0 + 1000.0,
                        Event::JoyX(rudder) => events[3] = rudder * 30.0,
                        Event::JoyY(aileron) => events[2] = aileron * 21.5,
                        Event::CamX(elevator) => events[1] = elevator * 25.0,
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

    pub fn thread_build(mut controller: Self) -> JoinHandle<()> {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let _control_task = rt.block_on(async move {
                loop {
                    let c = controller.send().await;
                    if let Command::Exit = c {
                        break;
                    }
                }
            });
        })
    }
}

impl AsInputer for StickController {
    fn get_receiver(&self) -> InputReceiver {
        self.receiver.clone()
    }
}
