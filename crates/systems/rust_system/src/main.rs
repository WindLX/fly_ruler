use clap::Parser;
use log::{error, info};
use once_cell::sync::Lazy;
use rust_system::{
    args::Args,
    handler::{server_handler, system_step_handler},
    lua::LuaManager,
    system::System,
    utils::{CancellationToken, Counter, Signal},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

static ARGS: Lazy<std::sync::Mutex<Args>> = Lazy::new(|| std::sync::Mutex::new(Args::parse()));

fn main() {
    env_logger::builder().init();

    let lua = LuaManager::new(&ARGS.lock().unwrap().config_path);

    let server_addr = lua.server_addr();
    let model_root_path = lua.model_root_path();
    let core_init_cfg = lua.core_init_cfg();
    let model_install_args = lua.model_install_args();
    let plane_init_cfg = lua.plane_init_cfg();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let mut system = System::new();
        system.set_dir(model_root_path);
        system.init(core_init_cfg);

        let keys: Vec<Uuid> = system.get_models().map_or_else(|e|{
            error!("{}",e);
            std::process::exit(1);
        }, |f|f).keys().cloned().collect();
        let models = system.get_models().map_or_else(|e|{
            error!("{}",e);
            std::process::exit(1);
        }, |f|f);
        for (index, k) in keys.iter().enumerate() {
            info!(
                "Id: {}, Model: {}",
                k.to_string(),
                models.get(k).unwrap().0.name,
            );
            if let Err(e) = system.enable_model(*k, &model_install_args[index]){
                error!("{}",e);
                std::process::exit(1);
            }
        }

        let f16_key = keys[0];

        let system = Arc::new(Mutex::new(system));
        let plane_counter = Counter::new();
        let run_signal = Signal::new();
        let global_cancellation_token = CancellationToken::new();

        tokio::select! {
            Err(e) = system_step_handler(system.clone(), run_signal.clone(), plane_counter.clone(), global_cancellation_token.clone()) => {
                error!("{}", e);
            },
            _ = server_handler(&server_addr, plane_init_cfg, system.clone(), plane_counter, run_signal, global_cancellation_token, f16_key) =>{
                error!("Server task finished");
            }
        }

        system.lock().await.err_stop();
    });
}
