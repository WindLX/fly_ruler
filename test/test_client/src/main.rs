use fly_ruler_codec::{
    Args, GetModelInfosResponse, PushPlaneRequest, RequestFrame, Response, ResponseFrame,
    ServiceCall,
};
use fly_ruler_utils::plane_model::Control;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::{net::TcpStream, sync::mpsc::channel};
use tokio_util::codec::{FramedRead, FramedWrite};

fn main() {
    env_logger::builder()
        .target(env_logger::Target::Stderr)
        .format_timestamp(None)
        .init();
    let init_cmd = Control {
        thrust: 5000.0,
        elevator: -0.09,
        aileron: 0.01,
        rudder: -0.01,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let stream = TcpStream::connect("127.0.0.1:2350").await.unwrap();

        let (reader, writer) = stream.into_split();
        let mut reader = FramedRead::new(reader, ResponseFrame);
        let mut writer = FramedWrite::new(writer, RequestFrame);

        let (tx, mut rx) = channel::<ServiceCall>(100);

        let client_writer_task = tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Some(data) => {
                        writer.send(data).await.unwrap();
                    }
                    None => break,
                }
            }
        });

        let main_task = tokio::spawn(async move {
            let call = ServiceCall {
                name: "GetModelInfos".to_string(),
                args: Some(Args::GetModelInfos),
            };
            tx.send(call).await.unwrap();

            loop {
                let response = reader.next().await;
                match response {
                    Some(response) => match response {
                        Ok(response) => {
                            if let Some(Response::GetModelInfos(GetModelInfosResponse {
                                model_infos,
                            })) = response.response
                            {
                                let f16_key = model_infos[0].clone().id;

                                let call = ServiceCall {
                                    name: "PushPlane".to_string(),
                                    args: Some(Args::PushPlane(PushPlaneRequest {
                                        model_id: f16_key,
                                        plane_init_cfg: None,
                                    })),
                                };
                                tx.send(call).await.unwrap();
                                let tx1 = tx.clone();

                                let plane_id = reader.next().await.unwrap().unwrap();
                                dbg!(plane_id);
                                let plane_id = reader.next().await.unwrap().unwrap();
                                if let Some(Response::PushPlane(response)) = plane_id.response {
                                    let plane_id = response.plane_id;
                                    let h1 = tokio::spawn(async move {
                                        loop {
                                            let response = reader.next().await;
                                            if let None = response {
                                                tokio::task::yield_now().await;
                                            } else {
                                                println!("{:?}", response.unwrap())
                                            };
                                        }
                                    });

                                    let h2 = tokio::spawn(async move {
                                        let mut count = 0;
                                        loop {
                                            let call = ServiceCall {
                                                name: "SendControl".to_string(),
                                                args: Some(Args::SendControl(
                                                    fly_ruler_codec::SendControlRequest {
                                                        plane_id: plane_id.clone(),
                                                        control: Some(init_cmd.clone()),
                                                    },
                                                )),
                                            };
                                            tx.send(call).await.unwrap();
                                            tokio::time::sleep(Duration::from_millis(10)).await;
                                            count += 1;
                                            if count == 100 {
                                                let call = ServiceCall {
                                                    name: "Tick".to_string(),
                                                    args: Some(Args::Tick),
                                                };
                                                tx1.send(call).await.unwrap();
                                                dbg!("tick");
                                                count = 0;
                                            }
                                        }
                                    });

                                    h1.await.unwrap();
                                    h2.await.unwrap();
                                    break;
                                }
                            } else {
                                continue;
                            }
                        }
                        Err(e) => {
                            dbg!(e);
                        }
                    },
                    None => {
                        tokio::task::yield_now().await;
                    }
                }
            }
        });

        main_task.await.unwrap();
        client_writer_task.await.unwrap();
    });
}
