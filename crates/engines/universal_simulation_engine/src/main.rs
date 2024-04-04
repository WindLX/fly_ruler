use clap::Parser;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{event, Level};
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};
use universal_simulation_engine::{
    args::Args,
    handler::{server_handler, system_step_handler},
    lua::LuaManager,
    system::System,
    utils::{CancellationToken, Signal},
};
use uuid::Uuid;

static ARGS: Lazy<std::sync::Mutex<Args>> = Lazy::new(|| std::sync::Mutex::new(Args::parse()));

fn main() {
    let lua = LuaManager::new(&ARGS.lock().unwrap().config_path);

    let server_addr = lua.server_addr();
    let tick_timeout = lua.tick_timeout();
    let read_rate = lua.read_rate();
    let is_block = lua.is_block();
    let model_root_path = lua.model_root_path();
    let core_init_cfg = lua.core_init_cfg();
    let model_install_args = lua.model_install_args();
    let plane_init_cfg = lua.plane_init_cfg();
    let log_filter = lua.log_filter();
    let log_dir = lua.log_dir();
    let log_file = lua.log_file();

    let env_filter = EnvFilter::new(log_filter);
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);

    let file_appender = rolling::never(log_dir, log_file);
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    Registry::default()
        .with(env_filter)
        .with(ErrorLayer::default())
        .with(formatting_layer)
        .with(file_layer)
        .init();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let mut system = System::new();
        system.set_dir(model_root_path);
        system.init(core_init_cfg);

        let keys: Vec<Uuid> = system.get_models().map_or_else(|e|{
            event!(Level::ERROR,"{}",e);
            std::process::exit(1);
        }, |f|f).keys().cloned().collect();
        let models = system.get_models().map_or_else(|e|{
            event!(Level::ERROR,"{}",e);
            std::process::exit(1);
        }, |f|f);
        for (index, k) in keys.iter().enumerate() {
            event!(Level::INFO,
                "Id: {}, Model: {}",
                k.to_string(),
                models.get(k).unwrap().0.name,
            );
            if let Err(e) = system.enable_model(*k, &model_install_args[index]){
                event!(Level::ERROR,"{}",e);
                std::process::exit(1);
            }
        }

        let system = Arc::new(Mutex::new(system));
        let run_signal = Signal::new();
        let global_cancellation_token = CancellationToken::new();
        let gct = global_cancellation_token.clone();

        tokio::select! {
            Err(e) = system_step_handler(system.clone(), is_block, run_signal.clone(), global_cancellation_token.clone()) => {
                gct.cancel();
                event!(Level::ERROR,"{}", e);
            },
            _ = server_handler(&server_addr, tick_timeout, read_rate, plane_init_cfg, system.clone(), run_signal, global_cancellation_token) =>{
                gct.cancel();
                event!(Level::ERROR,"Server task finished");
            }
        }

        system.lock().await.err_stop();
    });
}
