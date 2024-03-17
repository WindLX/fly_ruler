use fly_ruler_core::{
    algorithm::nelder_mead::NelderMeadOptions,
    core::{CoreInitCfg, PlaneInitCfg},
    parts::trim::{TrimInit, TrimTarget},
};
use fly_ruler_utils::{
    plane_model::{Control, FlightCondition},
    Command,
};
use log::{error, info};
use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use uuid::Uuid;

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
        let stream = TcpStream::connect("127.0.0.1:2345").await.unwrap();
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
                let buf = serde_json::to_string(&init_cmd).unwrap();
                let buf = format!("{}\n", buf);
                info!("Send cmd successfully");
                writer.write_all(&buf.as_bytes()).await.unwrap();
                writer.flush().await.unwrap();
            }
        });

        viewer_task.await;
        controller_task.await;
    });
}
