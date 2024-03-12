use std::io::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    config
        .protoc_arg("--proto_path")
        .protoc_arg("./proto")
        .out_dir("src/generated")
        .compile_protos(
            &[
                "core_output.proto",
                "control.proto",
                "state.proto",
                "state_extend.proto",
            ],
            &["src"],
        )?;
    Ok(())
}
