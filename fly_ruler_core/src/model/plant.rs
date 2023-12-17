use fly_ruler_plugin::model::Model;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Plant {
    id: usize,
    group: usize,
    model: Arc<Mutex<Model>>,
    // controller: Arc<Mutex<dyn fly_ruler_plugin::controller::Controller>>,
}
