use fly_ruler_utils::{
    plane_model::{CoreOutput, ToCsv},
    IsViewer, OutputReceiver, ViewerCancellationToken,
};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub struct CSVViewer {
    path: PathBuf,
    receiver: Option<OutputReceiver>,
}

impl CSVViewer {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: PathBuf::from(path.as_ref()),
            receiver: None,
        }
    }

    fn open(&self) -> File {
        let mut file = File::create(&self.path).unwrap();
        file.write("time(s),".as_bytes()).unwrap();
        file.write(CoreOutput::titles().as_bytes()).unwrap();
        file.write("\n".as_bytes()).unwrap();
        file
    }

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
            let _viewer_task = rt.block_on(async move {
                let mut file = viewer.open();
                loop {
                    if cancellation_token.is_cancelled() {
                        break;
                    }
                    let o = viewer.recv().await;
                    match o {
                        Some(o) => {
                            file.write(format!("{:.2},", o.0).as_bytes()).unwrap();
                            file.write(o.1.data_string().as_bytes()).unwrap();
                            file.write(&[b'\n']).unwrap();
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

impl IsViewer for CSVViewer {
    fn set_receiver(&mut self, receiver: OutputReceiver) {
        self.receiver = Some(receiver);
    }
}
