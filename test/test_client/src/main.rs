use fly_ruler_codec::{
    Args, Decoder, Encoder, GetModelInfosResponse, ProtoCodec, PushPlaneRequest, Response,
    ServiceCall, ServiceCallResponse,
};
use fly_ruler_utils::plane_model::Control;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc::channel,
};

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

        let (mut reader, mut writer) = stream.into_split();
        let mut codec = ProtoCodec::new();

        let (tx, mut rx) = channel::<Vec<u8>>(100);

        let client_writer_task = tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Some(data) => {
                        writer.write_all(&data).await.unwrap();
                        writer.flush().await.unwrap();
                    }
                    None => break,
                }
            }
        });

        let main_task = tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            let call = ServiceCall {
                name: "GetModelInfos".to_string(),
                args: Some(Args::GetModelInfos),
            };
            tx.send(codec.encode(call).unwrap()).await.unwrap();

            loop {
                let n = reader.read(&mut buf).await;
                let response: ServiceCallResponse = codec.decode(&mut &buf[..n.unwrap()]).unwrap();
                println!("{:?}", response);
                if let Some(Response::GetModelInfos(GetModelInfosResponse { model_infos })) =
                    response.response
                {
                    let f16_key = model_infos[0].clone().id;

                    let call = ServiceCall {
                        name: "PushPlane".to_string(),
                        args: Some(Args::PushPlane(PushPlaneRequest {
                            model_id: f16_key,
                            plane_init_cfg: None,
                        })),
                    };
                    tx.send(codec.encode(call).unwrap()).await.unwrap();
                    let tx1 = tx.clone();

                    let h1 = tokio::spawn(async move {
                        loop {
                            let n = reader.read(&mut buf).await;
                            let response: ServiceCallResponse =
                                codec.decode(&mut &buf[..n.unwrap()]).unwrap();
                            println!("{:?}", response);
                        }
                    });

                    let h2 = tokio::spawn(async move {
                        loop {
                            let call = ServiceCall {
                                name: "SendControl".to_string(),
                                args: Some(Args::SendControl(
                                    fly_ruler_codec::SendControlRequest {
                                        control: Some(init_cmd.clone()),
                                    },
                                )),
                            };
                            tx.send(codec.encode(call).unwrap()).await.unwrap();
                            tokio::time::sleep(Duration::from_millis(15)).await;
                        }
                    });

                    let h3 = tokio::spawn(async move {
                        loop {
                            let call = ServiceCall {
                                name: "Tick".to_string(),
                                args: Some(Args::Tick),
                            };
                            tx1.send(codec.encode(call).unwrap()).await.unwrap();
                            tokio::time::sleep(Duration::from_millis(3000)).await;
                        }
                    });

                    h1.await.unwrap();
                    h2.await.unwrap();
                    h3.await.unwrap();
                    break;
                } else {
                    continue;
                }
            }
        });

        main_task.await.unwrap();
        client_writer_task.await.unwrap();
    });
}
