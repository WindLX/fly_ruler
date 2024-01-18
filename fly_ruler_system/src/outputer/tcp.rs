use fly_ruler_utils::{
    encode_view_message, plane_model::CoreOutput, IsOutputer, OutputReceiver,
    ViewerCancellationToken,
};

pub struct TcpViewer {
    host: String,
    port: u16,
    receiver: Option<OutputReceiver>,
}

impl TcpViewer {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
            receiver: None,
        }
    }

    fn open(&self) {}

    async fn recv(&mut self) -> Option<(f64, CoreOutput)> {
        match &mut self.receiver {
            Some(receiver) => receiver.receive().await,
            None => None,
        }
    }

    pub fn thread_build(
        mut viewer: Self,
        cancellation_token: ViewerCancellationToken,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async move {
                loop {
                    if cancellation_token.is_cancelled() {
                        break;
                    }
                    let o = viewer.recv().await;
                    match o {
                        Some(o) => {
                            let proto_msg = encode_view_message(o.0, &o.1);
                        }
                        None => {
                            break;
                        }
                    }
                }
            });
        })
    }
}

impl IsOutputer for TcpViewer {
    fn set_receiver(&mut self, receiver: OutputReceiver) {
        self.receiver = Some(receiver);
    }
}
