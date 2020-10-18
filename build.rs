fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .format(false)
        .compile(&["proto/health.proto"], &["proto"])
        .map_err(|err| err.into())
}
