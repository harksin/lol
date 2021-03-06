fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let OUT_DIR = "proto_generated";
    // std::fs::create_dir(OUT_DIR);
    // tonic_build::configure()
    //     .format(true)
    //     .build_server(false)
    //     .out_dir(OUT_DIR)
    //     .compile(&["proto/lol-bridge.proto"], &["proto"])?;

    let config = prost_build::Config::new();
    tonic_build::configure().compile_with_config(
        config,
        &["proto/lol-bridge.proto"],
        &["proto"],
    )?;

    Ok(())
}
