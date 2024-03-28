use std::io::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    config
        .protoc_arg("--proto_path")
        .protoc_arg("../../../proto")
        .out_dir("src/generated")
        .compile_protos(
            &[
                "service.proto",
                "id.proto",
                "plugin.proto",
                "state_extern.proto",
                "core_output.proto",
                "control.proto",
                "state.proto",
                "state_extend.proto",
                "plane_init_cfg.proto",
            ],
            &["src"],
        )?;
    Ok(())
}
