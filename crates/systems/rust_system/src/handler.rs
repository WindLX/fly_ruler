use crate::{
    system::{SysError, System},
    utils::{CancellationToken, Counter, Signal},
};
use fly_ruler_codec::{Decoder, Encoder, PlaneMessage, ProtoCodec};
use fly_ruler_core::core::PlaneInitCfg;
use fly_ruler_utils::{error::FrError, Command, InputSender, OutputReceiver};
use log::{error, info, trace, warn};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    sync::{broadcast, Mutex},
};
use uuid::Uuid;

pub async fn system_step_handler(
    system: Arc<Mutex<System>>,
    run_signal: Signal,
    plane_counter: Counter,
    cancellation_token: CancellationToken,
) -> Result<(), SysError> {
    loop {
        if cancellation_token.is_cancelled() {
            break Ok(());
        }
        if plane_counter.get() >= 1 && run_signal.available() {
            system.lock().await.step().await?;
        } else {
            tokio::task::yield_now().await;
        }
    }
}

pub async fn server_handler(
    server_addr: &str,
    plane_init_cfg: PlaneInitCfg,
    plane_builder: Arc<Mutex<System>>,
    plane_counter: Counter,
    run_signal: Signal,
    cancellation_token: CancellationToken,
    f16_key: Uuid,
) {
    let listener = TcpListener::bind(server_addr).await.unwrap();
    let (client_channel_sender, _) = broadcast::channel::<Vec<u8>>(100);
    let codec = ProtoCodec::new();

    loop {
        let cancellation_token = cancellation_token.clone();
        if cancellation_token.is_cancelled() {
            break;
        }

        let plane_counter_1 = plane_counter.clone();
        let (client, client_addr) = listener.accept().await.unwrap();
        plane_counter.add();
        info!("accepted connection from {}", client_addr);

        let plane_remover = plane_builder.clone();
        run_signal.red();
        let r = plane_builder
            .lock()
            .await
            .push_plane(f16_key.clone(), plane_init_cfg)
            .await;
        if let Err(e) = r {
            error!("{}", e);
            cancellation_token.cancel();
            break;
        }
        let (id, viewer) = r.unwrap();
        let controller = plane_builder
            .lock()
            .await
            .set_controller(id.clone(), 10)
            .await;
        if let Err(e) = controller {
            error!("{}", e);
            cancellation_token.cancel();
            break;
        }
        let controller = controller.unwrap();

        info!("client {} connect Plane {}", client_addr, id);
        let client_channel_sender = client_channel_sender.clone();
        let client_channel_receiver = client_channel_sender.subscribe();
        let (client_reader, client_writer) = client.into_split();
        let group_cancellation_token = CancellationToken::new();
        let gct = group_cancellation_token.clone();

        let _cancel_task = tokio::spawn(async move {
            loop {
                if gct.is_cancelled() {
                    plane_remover.lock().await.remove_plane(id).await;
                    plane_counter_1.sub();
                    break;
                }
            }
        });

        let v_ct = cancellation_token.clone();
        let g_v_ct = group_cancellation_token.clone();
        let _viewer_task = tokio::spawn(async move {
            let r = viewer_handler(
                id,
                client_channel_sender,
                viewer,
                codec,
                v_ct.clone(),
                g_v_ct.clone(),
            )
            .await;
            if r.is_err() {
                g_v_ct.cancel();
                warn!("client: {} disconnected due to: {}", id, r.err().unwrap());
            }
        });

        let c_ct = cancellation_token.clone();
        let g_c_ct = group_cancellation_token.clone();
        let _client_task = tokio::spawn(async move {
            let r = client_write_handler(
                id,
                client_channel_receiver,
                client_writer,
                c_ct.clone(),
                g_c_ct.clone(),
            )
            .await;
            if r.is_err() {
                g_c_ct.cancel();
                warn!("client: {} disconnected due to: {}", id, r.err().unwrap());
            }
        });

        let co_ct = cancellation_token.clone();
        let g_co_ct = group_cancellation_token.clone();
        let _controller_task = tokio::spawn(async move {
            let r = controller_handler(
                id,
                client_reader,
                controller,
                codec,
                co_ct.clone(),
                g_co_ct.clone(),
            )
            .await;
            if r.is_err() {
                g_co_ct.cancel();
                warn!("client: {} disconnected due to: {}", id, r.err().unwrap());
            }
        });

        run_signal.green();
        tokio::task::yield_now().await;
    }
}

async fn viewer_handler(
    id: Uuid,
    client_channel_sender: broadcast::Sender<Vec<u8>>,
    mut viewer: OutputReceiver,
    mut codec: impl Encoder<PlaneMessage>,
    global_cancellation_token: CancellationToken,
    group_cancellation_token: CancellationToken,
) -> Result<(), FrError> {
    let sender = client_channel_sender.clone();
    let id = id;
    loop {
        if global_cancellation_token.is_cancelled() || group_cancellation_token.is_cancelled() {
            break;
        }
        viewer.changed().await?;
        let output = viewer.get_and_update();
        let chars = codec.encode(PlaneMessage {
            id: id.to_string(),
            time: output.0,
            output: output.1.clone(),
        })?;
        sender
            .send(chars)
            .map_err(|e| FrError::Sync(e.to_string()))?;
        trace!("received output from viewer: {}", id);
    }
    Ok(())
}

async fn client_write_handler(
    id: Uuid,
    mut client_channel_receiver: broadcast::Receiver<Vec<u8>>,
    client_writer: OwnedWriteHalf,
    global_cancellation_token: CancellationToken,
    group_cancellation_token: CancellationToken,
) -> Result<(), FrError> {
    let mut client_writer = BufWriter::new(client_writer);
    loop {
        if global_cancellation_token.is_cancelled() || group_cancellation_token.is_cancelled() {
            break;
        }
        let msg = client_channel_receiver
            .recv()
            .await
            .map_err(|e| FrError::Sync(e.to_string()))?;
        client_writer
            .write_all(msg.as_slice())
            .await
            .map_err(|e| FrError::Sync(e.to_string()))?;
        client_writer
            .flush()
            .await
            .map_err(|e| FrError::Sync(e.to_string()))?;
        trace!("client send successfully: {}", id);
    }
    Ok(())
}

async fn controller_handler(
    id: Uuid,
    client_reader: OwnedReadHalf,
    controller: InputSender,
    mut codec: impl Decoder<Command>,
    global_cancellation_token: CancellationToken,
    group_cancellation_token: CancellationToken,
) -> Result<(), FrError> {
    // let end_byte = b'\n';
    let mut last_cmd;
    let mut client_reader = BufReader::new(client_reader);
    loop {
        if global_cancellation_token.is_cancelled() || group_cancellation_token.is_cancelled() {
            break;
        }
        let mut buf = vec![0; 1024 * 10];
        let n = client_reader
            .read(&mut buf)
            .await
            .map_err(|e| FrError::Sync(e.to_string()))?;
        last_cmd = codec.decode(&buf[..n])?;
        controller.send(&last_cmd).await?;
        trace!("received command from client: {}", id);
    }
    Ok(())
}
