use crate::{
    system::System,
    utils::{CancellationToken, Signal},
};
use anyhow::{anyhow, Result};
use fly_ruler_codec::{
    Args, GetModelInfosResponse, PlaneMessage, PluginInfoTuple, PushPlaneResponse, RequestFrame,
    Response, ResponseFrame, ServiceCallResponse,
};
use fly_ruler_core::core::PlaneInitCfg;
use fly_ruler_utils::{InputSender, OutputReceiver};
use futures_util::{SinkExt, StreamExt};
use log::{debug, info, trace, warn};
use std::{collections::HashMap, net::SocketAddr, ops::Deref, sync::Arc, time::Duration};
use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    sync::{broadcast, mpsc, Mutex},
    time,
};
use tokio_util::codec::{FramedRead, FramedWrite};
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
        } else {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        tokio::task::yield_now().await;
    }
}

pub async fn server_handler(
    server_addr: &str,
    tick_timeout: u64,
    read_rate: u64,
    init_cfg: PlaneInitCfg,
    system: Arc<Mutex<System>>,
    run_signal: Signal,
    cancellation_token: CancellationToken,
) {
    let listener = TcpListener::bind(server_addr).await.unwrap();
    let (broadcast_channel_sender, _) = broadcast::channel::<ServiceCallResponse>(1024);
    info!("Server started on {}", server_addr);

    loop {
        let cancellation_token = cancellation_token.clone();
        if cancellation_token.is_cancelled() {
            break;
        }

        let (client, client_addr) = listener.accept().await.unwrap();
        info!("Accepted connection from {}", client_addr);

        let (reader, writer) = client.into_split();
        let reader = FramedRead::new(reader, RequestFrame);
        let writer = FramedWrite::new(writer, ResponseFrame);
        let controllers = Arc::new(Mutex::new(HashMap::new()));
        let grct = CancellationToken::new();
        let (private_channel_sender, private_channel_receiver) =
            mpsc::channel::<ServiceCallResponse>(1024);

        let _rpc_task = tokio::spawn({
            let controller1 = controllers.clone();
            let controller2 = controllers.clone();
            let gct1 = cancellation_token.clone();
            let grct1 = grct.clone();
            let grct2 = grct.clone();
            let system1 = system.clone();
            let broadcast_channel_sender1 = broadcast_channel_sender.clone();
            let broadcast_channel_sender2 = broadcast_channel_sender.clone();
            let run_signal1 = run_signal.clone();
            async move {
                let r = rpc_handler(
                    client_addr,
                    tick_timeout,
                    read_rate,
                    reader,
                    system1,
                    init_cfg,
                    broadcast_channel_sender1,
                    private_channel_sender,
                    controller1,
                    run_signal1,
                    gct1,
                    grct1,
                )
                .await;
                if let Err(e) = r {
                    grct2.cancel();
                    warn!("RPC Client: {} dropped, due to {}", client_addr, e);
                    for (id, _sender) in controller2.lock().await.deref().iter() {
                        let _ = broadcast_channel_sender2.send(ServiceCallResponse {
                            name: "LostPlane".to_string(),
                            response: Some(Response::LostPlane(id.to_string())),
                        });
                    }
                }
            }
        });

        let _client_write_task = tokio::task::spawn({
            let controller1 = controllers.clone();
            let gct1 = cancellation_token.clone();
            let grct1 = grct.clone();
            let grct2 = grct.clone();
            let broadcast_channel_receiver = broadcast_channel_sender.subscribe();
            let broadcast_channel_sender1 = broadcast_channel_sender.clone();
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
                    for (id, _sender) in controller1.lock().await.deref().iter() {
                        let _ = broadcast_channel_sender1.send(ServiceCallResponse {
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
    broadcast_channel_sender: broadcast::Sender<ServiceCallResponse>,
    mut viewer: OutputReceiver,
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
        let output = ServiceCallResponse {
            name: "Output".to_string(),
            response: Some(Response::Output(PlaneMessage {
                id: id.to_string(),
                time: output.0,
                output: Some(output.1.clone()),
            })),
        };
        sender.send(output)?;
        trace!("Received output from viewer: {}", id);

        tokio::task::yield_now().await;
    }
    Ok(())
}

async fn client_write_handler(
    ip: SocketAddr,
    mut broadcast_channel_receiver: broadcast::Receiver<ServiceCallResponse>,
    mut private_channel_receiver: mpsc::Receiver<ServiceCallResponse>,
    mut client_writer: FramedWrite<OwnedWriteHalf, ResponseFrame>,
    global_cancellation_token: CancellationToken,
    group_cancellation_token: CancellationToken,
) -> Result<()> {
    loop {
        if global_cancellation_token.is_cancelled() || group_cancellation_token.is_cancelled() {
            break;
        }

        // Try receiving from broadcast channel
        if let Ok(msg) = broadcast_channel_receiver.try_recv() {
            client_writer.send(msg).await?;
            debug!("Broadcast client: {} send successfully", ip);
        }

        // Try receiving from private channel
        if let Ok(msg) = private_channel_receiver.try_recv() {
            client_writer.send(msg).await?;
            debug!("Private client: {} send successfully", ip);
        }

        tokio::task::yield_now().await;
    }
    Ok(())
}

async fn rpc_handler(
    ip: SocketAddr,
    tick_timeout: u64,
    read_rate: u64,
    mut client_reader: FramedRead<OwnedReadHalf, RequestFrame>,
    system: Arc<Mutex<System>>,
    init_cfg: PlaneInitCfg,
    broadcast_channel_sender: broadcast::Sender<ServiceCallResponse>,
    private_channel_sender: mpsc::Sender<ServiceCallResponse>,
    controllers: Arc<Mutex<HashMap<String, InputSender>>>,
    run_signal: Signal,
    global_cancellation_token: CancellationToken,
    group_cancellation_token: CancellationToken,
) -> Result<()> {
    let mut last_tick = time::Instant::now();
    loop {
        if global_cancellation_token.is_cancelled() || group_cancellation_token.is_cancelled() {
            break;
        }

        let delta_time = time::Instant::now().duration_since(last_tick);
        if delta_time > Duration::from_millis(tick_timeout) {
            return Err(anyhow!("Client {} tick timeout", ip));
        }

        let mut count = 0;
        while count < read_rate {
            count += 1;
            let request = client_reader.next().await;
            match request {
                Some(call) => {
                    let call = call?;
                    match call.name.as_str() {
                        "GetModelInfos" => {
                            info!("Client: {} request `GetModelInfo`", ip);
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

                            private_channel_sender.send(response).await?;
                            info!("Client: {} request `GetModelInfo` reply", ip)
                        }
                        "PushPlane" => {
                            info!("Client: {} request `PushPlane`", ip);
                            run_signal.red();

                            let args = match call.args {
                                Some(Args::PushPlane(model_id)) => model_id,
                                _ => {
                                    let err = ServiceCallResponse {
                                        name: "PushPlane".to_string(),
                                        response: Some(Response::Error(
                                            "Invalid RPC args".to_string(),
                                        )),
                                    };
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
                            let id = r.0;
                            let controller = system.lock().await.set_controller(id, 30).await?;
                            controllers.lock().await.insert(id.to_string(), controller);

                            tokio::task::spawn({
                                let gct1 = global_cancellation_token.clone();
                                let grct1 = group_cancellation_token.clone();
                                let grct2 = group_cancellation_token.clone();
                                let broadcast_channel_sender1 = broadcast_channel_sender.clone();
                                let broadcast_channel_sender2 = broadcast_channel_sender.clone();
                                let controllers = controllers.clone();
                                async move {
                                    let rr = viewer_handler(
                                        r.0,
                                        broadcast_channel_sender1,
                                        r.1.clone(),
                                        gct1,
                                        grct1,
                                    )
                                    .await;
                                    if let Err(e) = rr {
                                        grct2.cancel();
                                        warn!("RPC Client: {} dropped, due to {}", ip, e);
                                        let response = ServiceCallResponse {
                                            name: "LostPlane".to_string(),
                                            response: Some(Response::LostPlane(r.0.to_string())),
                                        };
                                        controllers.lock().await.remove(&id.to_string());
                                        let _ = broadcast_channel_sender2.send(response);
                                    }
                                }
                            });

                            let response = ServiceCallResponse {
                                name: "PushPlane".to_string(),
                                response: Some(Response::PushPlane(PushPlaneResponse {
                                    plane_id: r.0.to_string(),
                                })),
                            };
                            private_channel_sender.send(response).await?;

                            let response = ServiceCallResponse {
                                name: "NewPlane".to_string(),
                                response: Some(Response::NewPlane(r.0.to_string())),
                            };
                            broadcast_channel_sender.send(response)?;

                            run_signal.green();
                            info!("Client: {} request `PushPlane` reply", ip)
                        }
                        "SendControl" => {
                            let control = match call.args {
                                Some(Args::SendControl(control)) => control,
                                _ => {
                                    let err = ServiceCallResponse {
                                        name: "SendControl".to_string(),
                                        response: Some(Response::Error(
                                            "Invalid RPC args".to_string(),
                                        )),
                                    };
                                    private_channel_sender.send(err).await?;
                                    warn!("Invalid RPC args from client: {}", ip);
                                    continue;
                                }
                            };
                            if let Some(ref controller) =
                                controllers.lock().await.get(&control.plane_id)
                            {
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
                            let err = ServiceCallResponse {
                                name: other.to_string(),
                                response: Some(Response::Error(format!(
                                    "Invalid RPC command: {}",
                                    other
                                ))),
                            };
                            private_channel_sender.send(err).await?;
                            warn!("Invalid RPC command from client: {}", ip);
                        }
                    }
                }
                None => {
                    break;
                }
            }
            tokio::task::yield_now().await;
        }
    }
    Ok(())
}
