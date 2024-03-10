// use fly_ruler_utils::{
//     input_channel, plane_model::Control, Command, InputReceiver, InputSender, IsInputer, ViewerCancellationToken
// };
// use log::{info, trace};
// use std::thread::JoinHandle;

// pub struct TcpController {
//     id: usize,
//     receiver: Option<InputReceiver>,
//     host: String,
//     port: u16,
// }

// impl TcpController {
//     pub async fn new(host: &str, port: u16) -> Self {
//         Self {
//             host: host.to_string(),
//             port,
//             receiver: None,
//         }
//     }

//     fn open(&self) {}

//     async fn recv(&mut self) -> Option<Command> {
//     match &mut self.receiver {
//         Some(receiver) => Some(receiver.receive().await),
//         None => None,
//     }
//     }

//     pub fn thread_build(mut controller: Self, cancellation_token: ViewerCancellationToken) -> JoinHandle<()> {
//         std::thread::spawn(move || {
//             let rt = tokio::runtime::Builder::new_current_thread()
//                 .enable_all()
//                 .build()
//                 .unwrap();
//             rt.block_on(async move { loop {
//                 if cancellation_token.is_cancelled() {
//                     break;
//                 }
//                 let c = controller.recv().await;
//                 match c {
//                     Some(c) => {
// let proto_msg =
//                     },None => {
//     break;
// }
//                 }
//             } })
//         })
//     }
// }

// impl IsInputer for TcpController {
//     fn get_receiver(&self) -> InputReceiver {
//         self.receiver.clone()
//     }
// }
