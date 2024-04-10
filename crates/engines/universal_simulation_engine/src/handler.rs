use crate::system::System;
use anyhow::{anyhow, Result};
use fly_ruler_codec::{
    Args, GetModelInfosResponse, PlaneMessage, PluginInfoTuple, PushPlaneResponse, RequestFrame,
    Response, ResponseFrame, ServiceCallResponse,
};
use fly_ruler_core::core::PlaneInitCfg;
use fly_ruler_utils::{CancellationToken, InputSender, OutputReceiver, Signal};
use futures_util::{SinkExt, StreamExt};
use std::{collections::HashMap, net::SocketAddr, ops::Deref, sync::Arc, time::Duration};
use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    sync::{broadcast, mpsc, Mutex, Notify},
};
use tokio_util::codec::{FramedRead, FramedWrite};
use tracing::{event, instrument, Level};
use uuid::Uuid;

#[instrument(skip(system, cancellation_token, init_cfg),level = Level::INFO)]
pub async fn server_handler(
    server_addr: &str,
    tick_timeout: u64,
    read_rate: u64,
    init_cfg: PlaneInitCfg,
    system: Arc<std::sync::Mutex<System>>,
    controller_buffer: usize,
    cancellation_token: CancellationToken,
) {
    let listener = TcpListener::bind(server_addr).await.unwrap();
    let (broadcast_channel_sender, _) = broadcast::channel::<ServiceCallResponse>(1024);
    event!(Level::INFO, "Server started on {}", server_addr);

    loop {
        let cancellation_token = cancellation_token.clone();
        if cancellation_token.is_cancelled() {
            break;
        }

        let (client, client_addr) = listener.accept().await.unwrap();
        event!(Level::INFO, "Accepted connection from {}", client_addr);

        let (reader, writer) = client.into_split();
        let reader = FramedRead::new(reader, RequestFrame);
        let writer = FramedWrite::new(writer, ResponseFrame);
        let controllers = Arc::new(Mutex::new(HashMap::new()));
        let grct = CancellationToken::new();
        let (private_channel_sender, private_channel_receiver) =
            mpsc::channel::<ServiceCallResponse>(1024);
        let tick_notify = Arc::new(Notify::new());
        let run_signal = Signal::new();

        let _rpc_task = tokio::spawn({
            let controller1 = controllers.clone();
            let controller2 = controllers.clone();
            let gct1 = cancellation_token.clone();
            let grct1 = grct.clone();
            let grct2 = grct.clone();
            let system1 = system.clone();
            let broadcast_channel_sender1 = broadcast_channel_sender.clone();
            let broadcast_channel_sender2 = broadcast_channel_sender.clone();
            let tick_notify1 = tick_notify.clone();
            let run_signal1 = run_signal.clone();
            async move {
                let r = rpc_handler(
                    client_addr,
                    tick_timeout,
                    read_rate,
                    reader,
                    system1,
                    init_cfg,
                    controller_buffer,
                    broadcast_channel_sender1,
                    private_channel_sender,
                    controller1,
                    run_signal1,
                    tick_notify1,
                    gct1,
                    grct1,
                )
                .await;
                if let Err(e) = r {
                    grct2.cancel();
                    event!(
                        Level::WARN,
                        "RPC Client: {} dropped, due to {}",
                        client_addr,
                        e
                    );
                    for (id, _sender) in controller2.lock().await.deref().iter() {
                        let _ = broadcast_channel_sender2.send(ServiceCallResponse {
                            name: "LostPlane".to_string(),
                            response: Some(Response::LostPlane(id.to_string())),
                        });
                    }
                    controller2.lock().await.clear();
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
                    event!(
                        Level::WARN,
                        "RPC Client: {} dropped, due to {}",
                        client_addr,
                        e
                    );
                    for (id, _sender) in controller1.lock().await.deref().iter() {
                        let _ = broadcast_channel_sender1.send(ServiceCallResponse {
                            name: "LostPlane".to_string(),
                            response: Some(Response::LostPlane(id.to_string())),
                        });
                    }
                    controller1.lock().await.clear();
                }
            }
        });

        let _tick_task = tokio::spawn({
            let controller1 = controllers.clone();
            let broadcast_channel_sender1 = broadcast_channel_sender.clone();
            let gct1 = cancellation_token.clone();
            let grct1 = grct.clone();
            let grct2 = grct.clone();
            let tick_notify1 = tick_notify.clone();
            let run_signal1 = run_signal.clone();
            async move {
                let r = tick_handler(
                    client_addr,
                    tick_timeout,
                    run_signal1,
                    tick_notify1,
                    gct1,
                    grct1,
                )
                .await;
                if let Err(e) = r {
                    grct2.cancel();
                    event!(
                        Level::WARN,
                        "RPC Client: {} dropped, due to {}",
                        client_addr,
                        e
                    );
                    for (id, _sender) in controller1.lock().await.deref().iter() {
                        let _ = broadcast_channel_sender1.send(ServiceCallResponse {
                            name: "LostPlane".to_string(),
                            response: Some(Response::LostPlane(id.to_string())),
                        });
                    }
                    controller1.lock().await.clear();
                }
            }
        });

        tokio::time::sleep(Duration::from_millis(100)).await;
        tokio::task::yield_now().await;
    }
}

#[instrument(skip(
    broadcast_channel_sender,
    viewer,
    global_cancellation_token,
    group_cancellation_token
),level = Level::INFO)]
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
        event!(Level::TRACE, "Received output from viewer: {}", id);

        tokio::task::yield_now().await;
    }
    Ok(())
}

#[tracing::instrument(skip(
    client_writer,
    private_channel_receiver,
    global_cancellation_token,
    group_cancellation_token
),level = Level::INFO)]
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
            event!(Level::DEBUG, "Broadcast client: {} send successfully", ip);
        }

        // Try receiving from private channel
        if let Ok(msg) = private_channel_receiver.try_recv() {
            client_writer.send(msg).await?;
            event!(Level::DEBUG, "Private client: {} send successfully", ip);
        }

        tokio::task::yield_now().await;
    }
    Ok(())
}

#[instrument(skip(
    system,
    broadcast_channel_sender,
    client_reader,
    private_channel_sender,
    controllers,
    run_signal,
    tick_notify,
    group_cancellation_token,
    global_cancellation_token,
    init_cfg
),level = Level::INFO)]
async fn rpc_handler(
    ip: SocketAddr,
    tick_timeout: u64,
    read_rate: u64,
    mut client_reader: FramedRead<OwnedReadHalf, RequestFrame>,
    system: Arc<std::sync::Mutex<System>>,
    init_cfg: PlaneInitCfg,
    controller_buffer: usize,
    broadcast_channel_sender: broadcast::Sender<ServiceCallResponse>,
    private_channel_sender: mpsc::Sender<ServiceCallResponse>,
    controllers: Arc<Mutex<HashMap<String, InputSender>>>,
    run_signal: Signal,
    tick_notify: Arc<Notify>,
    global_cancellation_token: CancellationToken,
    group_cancellation_token: CancellationToken,
) -> Result<()> {
    loop {
        if global_cancellation_token.is_cancelled() || group_cancellation_token.is_cancelled() {
            break;
        }

        let mut count = 0;
        while count < read_rate {
            count += 1;
            let request = client_reader.next().await;
            match request {
                Some(call) => {
                    let call = call?;
                    event!(
                        Level::INFO,
                        "Client: {ip} request `{request}`",
                        ip = ip,
                        request = call.name.as_str()
                    );
                    match call.name.as_str() {
                        "GetModelInfos" => {
                            let model_infos: Vec<_> = system
                                .lock()
                                .unwrap()
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
                        }
                        "PushPlane" => {
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
                                    continue;
                                }
                            };

                            let (id, viewer, controller, _handler) = tokio::task::spawn_blocking({
                                let system = system.clone();
                                let group_cancellation_token = group_cancellation_token.clone();
                                move || {
                                    system.lock().unwrap().push_plane(
                                        Uuid::parse_str(&args.model_id).unwrap(),
                                        controller_buffer,
                                        args.plane_init_cfg.map_or_else(|| init_cfg, |c| c.into()),
                                        group_cancellation_token,
                                    )
                                }
                            })
                            .await??;

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
                                        id,
                                        broadcast_channel_sender1,
                                        viewer.clone(),
                                        gct1,
                                        grct1,
                                    )
                                    .await;
                                    if let Err(e) = rr {
                                        grct2.cancel();
                                        event!(
                                            Level::WARN,
                                            "RPC Client: {} dropped, due to {}",
                                            ip,
                                            e
                                        );
                                        let response = ServiceCallResponse {
                                            name: "LostPlane".to_string(),
                                            response: Some(Response::LostPlane(id.to_string())),
                                        };
                                        controllers.lock().await.remove(&id.to_string());
                                        let _ = broadcast_channel_sender2.send(response);
                                    }
                                }
                            });

                            let response = ServiceCallResponse {
                                name: "PushPlane".to_string(),
                                response: Some(Response::PushPlane(PushPlaneResponse {
                                    plane_id: id.to_string(),
                                })),
                            };
                            private_channel_sender.send(response).await?;
                            tokio::time::sleep(Duration::from_millis(10)).await;

                            let response = ServiceCallResponse {
                                name: "NewPlane".to_string(),
                                response: Some(Response::NewPlane(id.to_string())),
                            };
                            broadcast_channel_sender.send(response)?;

                            run_signal.green();
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
                                    event!(Level::WARN, "Invalid RPC args from client: {}", ip);
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
                        }
                        "Tick" => {
                            tick_notify.notify_one();
                        }
                        "Disconnect" => {
                            group_cancellation_token.cancel();
                            return Err(anyhow!("Client {} request `Disconnect`", ip));
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
                            event!(Level::WARN, "Invalid RPC command from client: {}", ip);
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        tokio::task::yield_now().await;
    }
    Ok(())
}

#[instrument(skip(
    tick_notify,
    run_signal,
    group_cancellation_token,
    global_cancellation_token,
),level = Level::INFO)]
async fn tick_handler(
    ip: SocketAddr,
    tick_timeout: u64,
    run_signal: Signal,
    tick_notify: Arc<Notify>,
    global_cancellation_token: CancellationToken,
    group_cancellation_token: CancellationToken,
) -> Result<()> {
    loop {
        if global_cancellation_token.is_cancelled() || group_cancellation_token.is_cancelled() {
            break;
        }
        let err = tokio::time::timeout(Duration::from_millis(tick_timeout), tick_notify.notified())
            .await
            .map_err(|_| anyhow!("Client {} tick timeout", ip));
        if run_signal.available() {
            return err;
        }
    }
    Ok(())
}
