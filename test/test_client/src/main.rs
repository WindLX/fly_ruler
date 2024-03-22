use std::time::Duration;

use fly_ruler_codec::{Encoder, ProtoCodec};
use fly_ruler_utils::{plane_model::Control, Command};
use log::{error, info};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};

fn main() {
    env_logger::builder()
        .target(env_logger::Target::Stderr)
        .format_timestamp(None)
        .init();
    let init_cmd = Command::Control(Control {
        thrust: 5000.0,
        elevator: -0.09,
        aileron: 0.01,
        rudder: -0.01,
    });
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let stream = TcpStream::connect("127.0.0.1:2350").await.unwrap();
        let (reader, writer) = stream.into_split();
        let viewer_task = tokio::spawn(async move {
            let mut buf_reader = reader;
            loop {
                let mut buf = [0u8; 4096];
                match buf_reader.read(&mut buf).await {
                    Ok(n) => {
                        if n == 0 {
                            break;
                        }
                        info!("{}", String::from_utf8_lossy(&buf[..n]));
                    }
                    Err(e) => {
                        error!("{}", e);
                        break;
                    }
                }
            }
        });

        let controller_task = tokio::spawn(async move {
            let writer = writer;
            let mut writer = BufWriter::new(writer);
            loop {
                let mut codec = ProtoCodec::new();
                let buf = codec.encode(init_cmd.clone()).unwrap();
                info!("Send cmd successfully");
                writer.write_all(buf.as_slice()).await.unwrap();
                writer.flush().await.unwrap();
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });

        viewer_task.await;
        controller_task.await;
    });
}
