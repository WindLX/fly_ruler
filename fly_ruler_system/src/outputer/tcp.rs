use std::{io::Write, net::TcpStream};

use async_trait::async_trait;
use fly_ruler_utils::{
    encode_view_message, plane_model::CoreOutput, IsOutputer, OutputReceiver,
    ViewerCancellationToken,
};

pub struct TcpOutputer {
    host: String,
    port: u16,
    receiver: Option<OutputReceiver>,
}

impl TcpOutputer {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
            receiver: None,
        }
    }

    // async fn open(&mut self) {
    //     let stream = TcpStream::connect((self.host.as_str(), self.port))
    //         .await
    //         .unwrap();
    //     self.stream = Some(stream);
    // }

    // async fn recv(&mut self) -> Option<(f64, CoreOutput)> {
    //     match &mut self.receiver {
    //         Some(receiver) => receiver.receive().await,
    //         None => None,
    //     }
    // }

    // pub fn thread_build(
    //     mut viewer: Self,
    //     cancellation_token: ViewerCancellationToken,
    // ) -> std::thread::JoinHandle<()> {
    //     std::thread::spawn(move || {
    //         let rt = tokio::runtime::Builder::new_current_thread()
    //             .enable_all()
    //             .build()
    //             .unwrap();
    //         rt.block_on(async move {
    //             let stream = TcpStream::connect((viewer.host.as_str(), viewer.port))
    //                 .await
    //                 .unwrap();
    //             loop {
    //                 if cancellation_token.is_cancelled() {
    //                     break;
    //                 }
    //                 let o = viewer.recv().await;
    //                 match o {
    //                     Some(o) => {
    //                         let proto_msg = encode_view_message(o.0, &o.1);
    //                         stream.try_write(&proto_msg);
    //                     }
    //                     None => {
    //                         break;
    //                     }
    //                 }
    //             }
    //         });
    //     })
    // }
}

impl IsOutputer<TcpStream> for TcpOutputer {
    fn set_receiver(&mut self, receiver: OutputReceiver) {
        self.receiver = Some(receiver);
    }

    fn get_receiver(&self) -> Option<OutputReceiver> {
        self.receiver.clone()
    }

    fn open(&mut self) -> TcpStream {
        let stream = TcpStream::connect((self.host.as_str(), self.port)).unwrap();
        stream
    }

    fn medium_handler(medium: &mut TcpStream, time: f64, output: CoreOutput) {
        let proto_msg = encode_view_message(time, &output);
        let _ = medium.write(&proto_msg);
    }
}
