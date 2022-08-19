fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(&["proto/test.proto"], &["proto"])?;

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(&["proto/health.proto"], &["proto"])
        .map_err(|err| err.into())
}
