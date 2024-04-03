pub mod utils;

use fly_ruler_codec::{
    Args, GetModelInfosResponse, PlaneMessage, PushPlaneRequest, PushPlaneResponse, RequestFrame,
    Response, ResponseFrame, SendControlRequest, ServiceCall,
};
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use log::{error, trace};
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use python_runtime::{
    ControlWrapper, CoreOutputWrapper, FlightConditionWrapper, NelderMeadOptionsWrapper,
    PlaneInitCfgWrapper, PlaneMessageWrapper, PluginInfoTupleWrapper, PluginInfoWrapper,
    PluginStateWrapper, StateExtendWrapper, StateWrapper, TrimInitWrapper, TrimTargetWrapper,
    UuidWrapper,
};
use std::time::Duration;
use tokio::{net::TcpStream, sync, task::JoinHandle};
use tokio_util::codec::{FramedRead, FramedWrite};
use utils::CancellationToken;

lazy_static! {
    static ref RT: tokio::runtime::Runtime = {
        std::thread::spawn(|| RT.block_on(futures::future::pending::<()>()));
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    };
    static ref GUARD: tokio::runtime::EnterGuard<'static> = RT.enter();
}

#[pyclass]
pub struct PyClient {
    cancellation_token: CancellationToken,
    tick_tx: Option<sync::mpsc::Sender<ServiceCall>>,
    tasks: Vec<JoinHandle<()>>,
    request_sender: sync::mpsc::Sender<ServiceCall>,
    get_model_infos_receiver: sync::mpsc::Receiver<GetModelInfosResponse>,
    push_plane_receiver: sync::mpsc::Receiver<PushPlaneResponse>,
    output_receiver: sync::mpsc::Receiver<PlaneMessage>,
    lost_plane_receiver: sync::mpsc::Receiver<String>,
    new_plane_receiver: sync::mpsc::Receiver<String>,
    error_receiver: sync::mpsc::Receiver<String>,
}

#[pymethods]
impl PyClient {
    #[staticmethod]
    pub async fn new(host: String, port: u16) -> PyResult<Self> {
        let stream = TcpStream::connect(format!("{host}:{port}")).await?;

        let (reader, writer) = stream.into_split();
        let mut reader = FramedRead::new(reader, ResponseFrame);
        let mut writer = FramedWrite::new(writer, RequestFrame);
        let cancellation_token = CancellationToken::new();
        let (tx, mut rx) = sync::mpsc::channel::<ServiceCall>(256);
        let (tick_tx, mut tick_rx) = sync::mpsc::channel::<ServiceCall>(10);
        let (tx1, rx1) = sync::mpsc::channel::<GetModelInfosResponse>(10);
        let (tx2, rx2) = sync::mpsc::channel::<PushPlaneResponse>(10);
        let (tx3, rx3) = sync::mpsc::channel::<PlaneMessage>(100);
        let (tx4, rx4) = sync::mpsc::channel::<String>(100);
        let (tx5, rx5) = sync::mpsc::channel::<String>(100);
        let (tx6, rx6) = sync::mpsc::channel::<String>(100);

        let writer_task = {
            let w_ct1 = cancellation_token.clone();
            let w_ct2 = cancellation_token.clone();
            tokio::spawn(async move {
                let r: Result<Result<(), anyhow::Error>, tokio::task::JoinError> =
                    tokio::spawn(async move {
                        loop {
                            if w_ct1.is_cancelled() {
                                break;
                            }
                            if let Ok(msg) = rx.try_recv() {
                                writer.send(msg).await?;
                            }

                            if let Ok(msg) = tick_rx.try_recv() {
                                writer.send(msg).await?;
                            }
                        }
                        Ok(())
                    })
                    .await;
                match r {
                    Ok(r) => match r {
                        Ok(()) => {}
                        Err(e) => {
                            error!("{}", e);
                            w_ct2.cancel();
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        w_ct2.cancel();
                    }
                }
            })
        };

        let reader_task = {
            let r_ct1 = cancellation_token.clone();
            let r_ct2 = cancellation_token.clone();
            tokio::spawn(async move {
                let r: Result<Result<(), anyhow::Error>, tokio::task::JoinError> =
                    tokio::spawn(async move {
                        loop {
                            if r_ct1.is_cancelled() {
                                break;
                            }
                            let response = reader.next().await;
                            match response {
                                Some(response) => match response {
                                    Ok(response) => {
                                        if let Some(response) = response.response {
                                            match response {
                                                Response::GetModelInfos(r) => tx1.send(r).await?,
                                                Response::PushPlane(r) => tx2.send(r).await?,
                                                Response::Output(r) => tx3.send(r).await?,
                                                Response::LostPlane(r) => tx4.send(r).await?,
                                                Response::NewPlane(r) => tx5.send(r).await?,
                                                Response::Error(r) => tx6.send(r).await?,
                                                _ => {}
                                            }
                                        }
                                    }
                                    Err(e) => {}
                                },
                                None => {
                                    tokio::task::yield_now().await;
                                }
                            }
                        }
                        Ok(())
                    })
                    .await;
                match r {
                    Ok(r) => match r {
                        Ok(()) => {}
                        Err(e) => {
                            error!("{}", e);
                            r_ct2.cancel();
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        r_ct2.cancel();
                    }
                }
            })
        };

        let tasks = vec![writer_task, reader_task];

        Ok(Self {
            cancellation_token,
            tasks,
            tick_tx: Some(tick_tx),
            request_sender: tx,
            get_model_infos_receiver: rx1,
            push_plane_receiver: rx2,
            output_receiver: rx3,
            lost_plane_receiver: rx4,
            new_plane_receiver: rx5,
            error_receiver: rx6,
        })
    }

    pub async fn stop(&mut self) {
        self.cancellation_token.cancel();
        let mut count = 0;
        while let Some(task) = self.tasks.pop() {
            trace!("task {} stop start", count);
            let _ = task.abort();
            trace!("task {} stop successfully", count);
            count += 1;
        }
    }

    pub async fn get_model_infos(&mut self) -> PyResult<Vec<PluginInfoTupleWrapper>> {
        let call = ServiceCall {
            name: "GetModelInfos".to_string(),
            args: Some(Args::GetModelInfos),
        };
        self.request_sender
            .send(call)
            .await
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let r = self.get_model_infos_receiver.recv().await;
        match r {
            Some(r) => Ok(r
                .model_infos
                .into_iter()
                .map(PluginInfoTupleWrapper::from)
                .collect()),
            None => Err(PyRuntimeError::new_err("Failed to get model infos")),
        }
    }

    pub async fn push_plane(
        &mut self,
        arg: (UuidWrapper, Option<PlaneInitCfgWrapper>),
    ) -> PyResult<UuidWrapper> {
        let call = ServiceCall {
            name: "PushPlane".to_string(),
            args: Some(Args::PushPlane(PushPlaneRequest {
                model_id: arg.0 .0.to_string(),
                plane_init_cfg: arg.1.map(|c| PlaneInitCfgWrapper::into(c)),
            })),
        };
        self.request_sender
            .send(call)
            .await
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let r = self.push_plane_receiver.recv().await;
        match r {
            Some(r) => Ok(UuidWrapper::parse_str(&r.plane_id)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?),
            None => Err(PyRuntimeError::new_err("Failed to push plane")),
        }
    }

    pub async fn send_control(
        &mut self,
        arg: (UuidWrapper, Option<ControlWrapper>),
    ) -> PyResult<()> {
        let call = ServiceCall {
            name: "SendControl".to_string(),
            args: Some(Args::SendControl(SendControlRequest {
                plane_id: arg.0 .0.to_string(),
                control: arg.1.map(|o| o.into()),
            })),
        };
        self.request_sender
            .send(call)
            .await
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    pub fn tick(&mut self, tick_period: Option<u64>) -> PyResult<()> {
        let tick_tx = match self.tick_tx.take() {
            Some(t) => t,
            None => return Err(PyRuntimeError::new_err("Tick already running")),
        };
        let tick_task = {
            let t_ct1 = self.cancellation_token.clone();
            let t_ct2 = self.cancellation_token.clone();
            tokio::spawn(async move {
                let r: Result<Result<(), anyhow::Error>, tokio::task::JoinError> =
                    tokio::spawn(async move {
                        loop {
                            if t_ct1.is_cancelled() {
                                break;
                            }
                            let call = ServiceCall {
                                name: "Tick".to_string(),
                                args: Some(Args::Tick),
                            };
                            tick_tx.send(call).await?;
                            tokio::time::sleep(Duration::from_millis(tick_period.unwrap_or(1000)))
                                .await;
                        }
                        Ok(())
                    })
                    .await;
                match r {
                    Ok(r) => match r {
                        Ok(()) => {}
                        Err(e) => {
                            error!("{}", e);
                            t_ct2.cancel();
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        t_ct2.cancel();
                    }
                }
            })
        };
        self.tasks.push(tick_task);
        Ok(())
    }

    pub async fn output(&mut self) -> PyResult<PlaneMessageWrapper> {
        let r = self.output_receiver.recv().await;
        match r {
            Some(r) => Ok(r.into()),
            None => Err(PyRuntimeError::new_err("Output channel dropped")),
        }
    }

    pub async fn lost_plane(&mut self) -> PyResult<String> {
        let r = self.lost_plane_receiver.recv().await;
        match r {
            Some(r) => Ok(r),
            None => Err(PyRuntimeError::new_err("Lost Plane channel dropped")),
        }
    }

    pub async fn new_plane(&mut self) -> PyResult<String> {
        let r = self.new_plane_receiver.recv().await;
        match r {
            Some(r) => Ok(r),
            None => Err(PyRuntimeError::new_err("New Plane channel dropped")),
        }
    }

    pub async fn error(&mut self) -> PyResult<String> {
        let r = self.error_receiver.recv().await;
        match r {
            Some(r) => Ok(PyRuntimeError::new_err(r).to_string()),
            None => Err(PyRuntimeError::new_err("Error channel dropped")),
        }
    }
}

#[pymodule]
#[pyo3(name = "flyruler_py_client")]
fn fr_py_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _guard = &*GUARD;
    env_logger::builder().init();
    m.add_class::<PyClient>()?;
    m.add_class::<PlaneMessageWrapper>()?;
    m.add_class::<ControlWrapper>()?;
    m.add_class::<StateWrapper>()?;
    m.add_class::<StateExtendWrapper>()?;
    m.add_class::<CoreOutputWrapper>()?;
    m.add_class::<UuidWrapper>()?;
    m.add_class::<PluginInfoTupleWrapper>()?;
    m.add_class::<PluginInfoWrapper>()?;
    m.add_class::<PluginStateWrapper>()?;
    m.add_class::<TrimInitWrapper>()?;
    m.add_class::<TrimTargetWrapper>()?;
    m.add_class::<NelderMeadOptionsWrapper>()?;
    m.add_class::<FlightConditionWrapper>()?;
    m.add_class::<PlaneInitCfgWrapper>()?;
    Ok(())
}
