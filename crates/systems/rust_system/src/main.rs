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
use rust_system::{
    encoder::{decode, encode},
    system::System,
};
use std::{
    collections::{BTreeSet, HashMap},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{broadcast, mpsc, Mutex, Notify},
};
use uuid::Uuid;

fn main() {
    env_logger::builder()
        .target(env_logger::Target::Stderr)
        .format_timestamp(None)
        .init();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:2345").await.unwrap();

        let mut system = System::new();
        system.set_dir("./models");
        system.init(CoreInitCfg {
            time_scale: Some(1.0),
            sample_time: Some(100),
        });

        let keys: Vec<Uuid> = system.get_models().keys().cloned().collect();
        let models = system.get_models();
        for k in &keys {
            system.enable_model(*k, &["./models/f16_model/data"]);
            info!(
                "Id: {}, Model: {}",
                k.to_string(),
                models.get(k).unwrap().0.name,
            );
        }

        let f16_key = keys[0];
        let plane_init_cfg = PlaneInitCfg {
            deflection: Some([0.0, 0.0, 0.0]),
            trim_target: TrimTarget {
                altitude: 15000.0,
                velocity: 500.0,
            },
            trim_init: Some(TrimInit {
                alpha: 8.49,
                control: Control {
                    thrust: 5000.0,
                    elevator: -0.09,
                    aileron: 0.01,
                    rudder: -0.01,
                },
            }),
            flight_condition: Some(FlightCondition::WingsLevel),
            optim_options: Some(NelderMeadOptions {
                max_fun_evals: 50000,
                max_iter: 10000,
                tol_fun: 1e-10,
                tol_x: 1e-10,
            }),
        };
        let init_cmd = Command::Control(Control {
            thrust: 5000.0,
            elevator: -0.09,
            aileron: 0.01,
            rudder: -0.01,
        });

        let system = Arc::new(Mutex::new(system));
        let system_one = system.clone();

        let notify = Arc::new(Notify::new());
        let notify_one = notify.clone();

        let go = Arc::new(AtomicBool::new(true));
        let go_sender = go.clone();

        let system_task = tokio::spawn(async move {
            notify_one.notified().await;
            loop {
                if go.load(Ordering::Relaxed) == true {
                    system_one.lock().await.step().await
                } else {
                    tokio::task::yield_now().await;
                }
            }
        });

        let (client_channel_sender, client_channel_receiver) = broadcast::channel::<String>(100);
        let main_task = tokio::spawn(async move {
            let mut one = false;
            loop {
                let (client, client_addr) = listener.accept().await.unwrap();
                info!("Accepted connection from {}", client_addr);

                if !one {
                    notify.notify_one();
                    one = true;
                }

                go_sender.store(false, Ordering::Relaxed);
                let (id, mut viewer) = system
                    .lock()
                    .await
                    .push_plane(f16_key.clone(), plane_init_cfg.clone())
                    .await
                    .unwrap();
                let controller = system.lock().await.set_controller(id.clone(), 10);
                go_sender.store(true, Ordering::Relaxed);

                info!("Client {} connect Plane {}", client_addr, id);
                let client_channel_sender = client_channel_sender.clone();
                let client_channel_receiver = client_channel_sender.subscribe();
                let (client_reader, client_writer) = client.into_split();

                let viewer_task = tokio::spawn(async move {
                    let sender = client_channel_sender.clone();
                    let id = id;
                    loop {
                        if viewer.has_changed().unwrap() {
                            let output = viewer.get_and_update();
                            let chars = format!(
                                "{}\n",
                                encode(output.0, vec![(id.to_string(), output.1)]).unwrap()
                            );
                            sender.send(chars).unwrap();
                        } else {
                            tokio::task::yield_now().await;
                        }
                    }
                });

                let client_task = tokio::spawn(async move {
                    let mut receiver = client_channel_receiver;
                    let mut client = client_writer;
                    let id = id;
                    loop {
                        let msg = receiver.recv().await.unwrap();
                        let r = client.write_all(msg.as_bytes()).await;
                        client.flush().await.unwrap();
                    }
                });

                let controller_task = {
                    let init_cmd = init_cmd.clone();
                    tokio::spawn(async move {
                        let byte = b'\n';
                        let controller = controller.unwrap();
                        let mut client = client_reader;
                        let mut last_cmd;
                        loop {
                            client.readable().await.unwrap();
                            let mut buf = [0; 1024];
                            let mut n = 0;
                            loop {
                                let b = client.read_u8().await.unwrap();
                                if b == byte {
                                    break;
                                }
                                buf[n] = b;
                                n += 1;
                            }
                            last_cmd = decode(&buf[..n]).unwrap();
                            controller.send(&last_cmd).await.unwrap();
                        }
                    })
                };
                tokio::task::yield_now().await;
            }
        });

        main_task.await;
    });
}
