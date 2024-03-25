use crate::{
    system::System,
    utils::{CancellationToken, Signal},
};
use anyhow::{anyhow, Result};
use fly_ruler_codec::{
    Args, Decoder, Encoder, GetModelInfosResponse, PlaneMessage, PluginInfoTuple, ProtoCodec,
    PushPlaneResponse, Response, ServiceCall, ServiceCallResponse,
};
use fly_ruler_core::core::PlaneInitCfg;
use fly_ruler_utils::{InputSender, OutputReceiver};
use log::{debug, info, trace, warn};
use std::{net::SocketAddr, ops::Deref, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    sync::{broadcast, mpsc, Mutex},
    time,
};
use uuid::Uuid;

pub async fn system_step_handler(
    system: Arc<Mutex<System>>,
    is_block: bool,
    run_signal: Signal,
    cancellation_token: CancellationToken,
) -> Result<()> {
    loop {
        if cancellation_token.is_cancelled() {
            break Ok(());
        }
        let c = system.lock().await.plane_count()?;
        if c >= 1 && run_signal.available() {
            let r = system.lock().await.step(is_block).await?;
            if let Err(e) = r {
                warn!("System step: {}", e);
            }
            debug!("System step one");
        }
        tokio::task::yield_now().await;
    }
}

pub async fn server_handler(
    server_addr: &str,
    tick_timeout: u64,
    init_cfg: PlaneInitCfg,
    system: Arc<Mutex<System>>,
    run_signal: Signal,
    cancellation_token: CancellationToken,
) {
    let listener = TcpListener::bind(server_addr).await.unwrap();
    let (broadcast_channel_sender, _) = broadcast::channel::<Vec<u8>>(1024);

    loop {
        let cancellation_token = cancellation_token.clone();
        if cancellation_token.is_cancelled() {
            break;
        }

        let (client, client_addr) = listener.accept().await.unwrap();
        info!("Accepted connection from {}", client_addr);

        let (reader, writer) = client.into_split();
        let plane_id = Arc::new(Mutex::new(None));
        let grct = CancellationToken::new();
        let (private_channel_sender, private_channel_receiver) = mpsc::channel::<Vec<u8>>(16);

        let _rpc_task = tokio::spawn({
            let plane_id1 = plane_id.clone();
            let plane_id2 = plane_id.clone();
            let gct1 = cancellation_token.clone();
            let grct1 = grct.clone();
            let grct2 = grct.clone();
            let system1 = system.clone();
            let broadcast_channel_sender1 = broadcast_channel_sender.clone();
            let run_signal1 = run_signal.clone();
            async move {
                let r = rpc_handler(
                    client_addr,
                    tick_timeout,
                    reader,
                    system1,
                    init_cfg,
                    broadcast_channel_sender1,
                    private_channel_sender,
                    plane_id1,
                    run_signal1,
                    gct1,
                    grct1,
                )
                .await;
                if let Err(e) = r {
                    grct2.cancel();
                    warn!("RPC Client: {} dropped, due to {}", client_addr, e);
                    if let Some(id) = plane_id2.lock().await.deref() {
                        let _ = ProtoCodec::new().encode(ServiceCallResponse {
                            name: "LostPlane".to_string(),
                            response: Some(Response::LostPlane(id.to_string())),
                        });
                    }
                }
            }
        });

        let _client_write_task = tokio::task::spawn({
            let plane_id1 = plane_id.clone();
            let gct1 = cancellation_token.clone();
            let grct1 = grct.clone();
            let grct2 = grct.clone();
            let broadcast_channel_receiver = broadcast_channel_sender.subscribe();
            async move {
                let r = client_write_handler(
                    client_addr,
                    broadcast_channel_receiver,
                    private_channel_receiver,
                    writer,
                    gct1,
                    grct1,
                )
                .await;
                if let Err(e) = r {
                    grct2.cancel();
                    warn!("RPC Client: {} dropped, due to {}", client_addr, e);
                    if let Some(id) = plane_id1.lock().await.deref() {
                        let _ = ProtoCodec::new().encode(ServiceCallResponse {
                            name: "LostPlane".to_string(),
                            response: Some(Response::LostPlane(id.to_string())),
                        });
                    }
                }
            }
        });

        tokio::time::sleep(Duration::from_millis(100)).await;
        tokio::task::yield_now().await;
    }
}

async fn viewer_handler(
    id: Uuid,
    broadcast_channel_sender: broadcast::Sender<Vec<u8>>,
    mut viewer: OutputReceiver,
    mut codec: impl Encoder<ServiceCallResponse>,
    global_cancellation_token: CancellationToken,
    group_cancellation_token: CancellationToken,
) -> Result<()> {
    let sender = broadcast_channel_sender.clone();
    let id = id;
    loop {
        if global_cancellation_token.is_cancelled() || group_cancellation_token.is_cancelled() {
            break;
        }
        viewer.changed().await?;
        let output = viewer.get_and_update();
        let chars = codec.encode(ServiceCallResponse {
            name: "Output".to_string(),
            response: Some(Response::Output(PlaneMessage {
                id: id.to_string(),
                time: output.0,
                output: Some(output.1.clone()),
            })),
        })?;
        sender.send(chars)?;
        trace!("Received output from viewer: {}", id);

        // tokio::time::sleep(Duration::from_millis(5)).await;
        tokio::task::yield_now().await;
    }
    Ok(())
}

async fn client_write_handler(
    ip: SocketAddr,
    mut broadcast_channel_receiver: broadcast::Receiver<Vec<u8>>,
    mut private_channel_receiver: mpsc::Receiver<Vec<u8>>,
    client_writer: OwnedWriteHalf,
    global_cancellation_token: CancellationToken,
    group_cancellation_token: CancellationToken,
) -> Result<()> {
    let mut client_writer = BufWriter::new(client_writer);
    loop {
        if global_cancellation_token.is_cancelled() || group_cancellation_token.is_cancelled() {
            break;
        }

        // Try receiving from broadcast channel
        if let Ok(msg) = broadcast_channel_receiver.try_recv() {
            client_writer.write_all(msg.as_slice()).await?;
            client_writer.flush().await?;
            debug!("Broadcast client: {} send successfully", ip);
        }

        // Try receiving from private channel
        if let Ok(msg) = private_channel_receiver.try_recv() {
            client_writer.write_all(msg.as_slice()).await?;
            client_writer.flush().await?;
            debug!("Private client: {} send successfully", ip);
        }

        // tokio::time::sleep(Duration::from_millis(5)).await;
        tokio::task::yield_now().await;
    }
    Ok(())
}

async fn rpc_handler(
    ip: SocketAddr,
    tick_timeout: u64,
    client_reader: OwnedReadHalf,
    system: Arc<Mutex<System>>,
    init_cfg: PlaneInitCfg,
    broadcast_channel_sender: broadcast::Sender<Vec<u8>>,
    private_channel_sender: mpsc::Sender<Vec<u8>>,
    plane_id: Arc<Mutex<Option<Uuid>>>,
    run_signal: Signal,
    global_cancellation_token: CancellationToken,
    group_cancellation_token: CancellationToken,
) -> Result<()> {
    let mut codec = ProtoCodec::new();
    let mut client_reader = BufReader::new(client_reader);

    let mut controller: Option<InputSender> = None;
    let mut last_tick = time::Instant::now();
    loop {
        if global_cancellation_token.is_cancelled() || group_cancellation_token.is_cancelled() {
            break;
        }
        let mut buf = vec![0; 1024 * 10];
        let delta_time = time::Instant::now().duration_since(last_tick);
        if delta_time > Duration::from_millis(tick_timeout) {
            return Err(anyhow!("Client {} tick timeout", ip));
        }
        let n = client_reader.read(&mut buf).await?;
        if n > 0 {
            let call: ServiceCall = codec.decode(&buf[..n])?;
            match call.name.as_str() {
                "GetModelInfos" => {
                    let model_infos: Vec<_> = system
                        .lock()
                        .await
                        .get_models()?
                        .into_iter()
                        .map(|m| PluginInfoTuple {
                            id: m.0.to_string(),
                            info: m.1 .0.into(),
                            state: m.1 .1.into(),
                        })
                        .collect();
                    let response = ServiceCallResponse {
                        name: "GetModelInfos".to_string(),
                        response: Some(Response::GetModelInfos(GetModelInfosResponse {
                            model_infos,
                        })),
                    };

                    // tokio::time::sleep(Duration::from_millis(5)).await;
                    let response_bytes = codec.encode(response)?;
                    private_channel_sender.send(response_bytes).await?;
                    info!("Client: {} request `GetModelInfo`", ip)
                }
                "PushPlane" => {
                    if plane_id.lock().await.is_none() {
                        run_signal.red();

                        let args = match call.args {
                            Some(Args::PushPlane(model_id)) => model_id,
                            _ => {
                                let err = codec.encode(ServiceCallResponse {
                                    name: "PushPlane".to_string(),
                                    response: Some(Response::Error("Invalid RPC args".to_string())),
                                })?;
                                private_channel_sender.send(err).await?;
                                warn!("Invalid RPC args from client: {}", ip);
                                continue;
                            }
                        };
                        let r = system
                            .lock()
                            .await
                            .push_plane(
                                Uuid::parse_str(&args.model_id)?,
                                None,
                                args.plane_init_cfg.map_or_else(|| init_cfg, |c| c.into()),
                            )
                            .await?;
                        *plane_id.lock().await = Some(r.0);
                        controller = Some(system.lock().await.set_controller(r.0, 30).await?);

                        // tokio::time::sleep(Duration::from_millis(5)).await;
                        tokio::task::spawn({
                            let gct1 = global_cancellation_token.clone();
                            let grct1 = group_cancellation_token.clone();
                            let grct2 = group_cancellation_token.clone();
                            let broadcast_channel_sender1 = broadcast_channel_sender.clone();
                            async move {
                                let rr = viewer_handler(
                                    r.0,
                                    broadcast_channel_sender1,
                                    r.1.clone(),
                                    codec,
                                    gct1,
                                    grct1,
                                )
                                .await;
                                if let Err(e) = rr {
                                    grct2.cancel();
                                    warn!("RPC Client: {} dropped, due to {}", ip, e);
                                    let _ = codec.encode(ServiceCallResponse {
                                        name: "LostPlane".to_string(),
                                        response: Some(Response::LostPlane(r.0.to_string())),
                                    });
                                }
                            }
                        });

                        // tokio::time::sleep(Duration::from_millis(5)).await;
                        let response = ServiceCallResponse {
                            name: "PushPlane".to_string(),
                            response: Some(Response::PushPlane(PushPlaneResponse {
                                plane_id: r.0.to_string(),
                            })),
                        };
                        let response_bytes = codec.encode(response)?;
                        private_channel_sender.send(response_bytes).await?;

                        tokio::time::sleep(Duration::from_millis(5)).await;
                        let response = ServiceCallResponse {
                            name: "NewPlane".to_string(),
                            response: Some(Response::NewPlane(r.0.to_string())),
                        };
                        let response_bytes = codec.encode(response)?;
                        broadcast_channel_sender.send(response_bytes)?;

                        run_signal.green();
                        info!("Client: {} request `PushPlane`", ip)
                    }
                }
                "SendControl" => {
                    if let Some(ref controller) = controller {
                        let control = match call.args {
                            Some(Args::SendControl(control)) => control,
                            _ => {
                                let err = codec.encode(ServiceCallResponse {
                                    name: "SendControl".to_string(),
                                    response: Some(Response::Error("Invalid RPC args".to_string())),
                                })?;
                                private_channel_sender.send(err).await?;
                                warn!("Invalid RPC args from client: {}", ip);
                                continue;
                            }
                        };
                        controller
                            .send(&control.control.unwrap_or_default())
                            .await?;
                    }
                    info!("Client: {} request `SendControl`", ip)
                }
                "Tick" => {
                    last_tick = time::Instant::now();
                    debug!("Client: {} request `Tick`", ip)
                }
                other => {
                    let err = codec.encode(ServiceCallResponse {
                        name: other.to_string(),
                        response: Some(Response::Error(format!("Invalid RPC command: {}", other))),
                    })?;
                    private_channel_sender.send(err).await?;
                    warn!("Invalid RPC command from client: {}", ip);
                    // tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }
        } else {
            // tokio::time::sleep(Duration::from_millis(10)).await;
        }
        tokio::task::yield_now().await;
    }
    Ok(())
}
