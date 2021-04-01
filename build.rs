fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(true)
        .compile(&["proto/health.proto"], &["proto"])
        .map_err(|err| err.into())
}
