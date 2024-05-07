use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=proto");
    build_proto_v0_37();
    build_vergen()?;
    Ok(())
}

fn build_proto_v0_37() {
    tonic_build::configure()
        .out_dir("src/prost/v0_37")
        .compile(&["v0_37/terndermint/abci/types.proto"], &["v0_37"])
        .expect("Failed to compile proto(s)");
}

fn build_vergen() -> Result<(), Box<dyn Error>> {
    // Emit the instructions
    EmitBuilder::builder()
        .git_sha(true)
        .build_timestamp()
        .cargo_features()
        .cargo_target_triple()
        .emit()?;
    Ok(())
}
