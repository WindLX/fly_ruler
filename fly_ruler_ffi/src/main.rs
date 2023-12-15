use fly_ruler_ffi::model::{callback, ModelLoader};

fn main() {
    let model = ModelLoader::new("f16", "./f16_model.dll");
    let mut model = model.unwrap();
    model.load().unwrap();
    model.register_logger(callback).unwrap();
    model.model_install("./data").unwrap();
    model.model_uninstall().unwrap();
}
