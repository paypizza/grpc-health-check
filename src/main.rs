#![forbid(unsafe_code)]
use error::{Error, ErrorKind};
use health_service::{health_client::HealthClient, serving_status, HealthCheckRequest};
use input::{Config, Opts};
#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
use std::fs;
use std::time::Duration;
use tonic::{
    transport::{Certificate, Channel, ClientTlsConfig, Identity},
    Request,
};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod error;
mod input;

mod health_service {
    #![allow(clippy::derive_partial_eq_without_eq)]

    tonic::include_proto!("grpc.health.v1");

    use health_check_response::ServingStatus;

    pub fn serving_status(status: i32) -> Result<(), Box<dyn std::error::Error>> {
        match ServingStatus::from_i32(status).unwrap_or(ServingStatus::Unknown) {
            ServingStatus::Serving => Ok(()),
            status => Err(format!("{:?}", status).into()),
        }
    }
}

/// grpc-health-check
#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build runtime");

    let local = tokio::task::LocalSet::new();
    local.block_on(&rt, run())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut opts: Opts = Opts::new();

    if opts.config.is_some() {
        let content = fs::read_to_string(&opts.config.as_ref().unwrap()).map_err(Error::from_io)?;

        let config = Config::parse_yaml(&content)
            .map_err(|e| Error::new(ErrorKind::InvalidConfig).with(e))?;

        opts.merge(&config)
    }

    if opts.port.is_none() {
        return Err(Error::new(ErrorKind::InvalidConfig)
            .with("undefined port")
            .into());
    }

    if opts.verbose {
        println!("{:?}", &opts);
    }

    let uri = format!(
        "{}://{}:{}",
        match opts.tls_ca_cert.is_some() {
            true => "https",
            false => "http",
        },
        opts.address,
        opts.port.unwrap()
    );

    let endpoint = Channel::from_shared(uri).map_err(Error::from_http)?;

    if opts.verbose {
        println!("uri: {}", endpoint.uri());
    }

    let channel = match opts.tls_ca_cert.is_some() {
        true => {
            let server_root_ca_cert =
                fs::read(opts.tls_ca_cert.unwrap()).map_err(Error::from_io)?;
            let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);

            let tls = match opts.tls_client_cert.is_some() || opts.tls_client_key.is_some() {
                true => {
                    if opts.tls_client_cert.is_none() {
                        return Err(Error::new(ErrorKind::InvalidConfig)
                            .with("tls client certificate: empty file path")
                            .into());
                    } else if opts.tls_client_key.is_none() {
                        return Err(Error::new(ErrorKind::InvalidConfig)
                            .with("tls client key: empty file path")
                            .into());
                    }

                    let client_cert =
                        fs::read(opts.tls_client_cert.unwrap()).map_err(Error::from_io)?;
                    let client_key =
                        fs::read(opts.tls_client_key.unwrap()).map_err(Error::from_io)?;
                    let client_identity = Identity::from_pem(client_cert, client_key);

                    ClientTlsConfig::new().identity(client_identity)
                }
                false => ClientTlsConfig::new(),
            }
            .domain_name(opts.tls_domain_name.unwrap_or_default())
            .ca_certificate(server_root_ca_cert);

            endpoint.tls_config(tls)?
        }
        false => endpoint,
    }
    .timeout(Duration::from_millis(opts.rpc_timeout))
    .user_agent(opts.user_agent.unwrap_or_default())?
    .connect()
    .await
    .map_err(Error::from_transport)?;

    // Create a new gRPC client.
    let mut client = HealthClient::new(channel);

    let request = Request::new(HealthCheckRequest {
        service: opts.service_name.unwrap_or_default(),
    });

    if opts.stream.is_some() && opts.stream.unwrap() {
        let mut stream = client
            .watch(request)
            .await
            .map_err(Error::from_transport)?
            .into_inner();

        while let Some(message) = stream.message().await? {
            if opts.verbose {
                println!("{:?}", message);
            }

            println!("{:?}", serving_status(message.status).unwrap())
        }

        Ok(())
    } else {
        let response = client.check(request).await.map_err(Error::from_transport)?;

        let message = response.into_inner();

        if opts.verbose {
            println!("{:?}", message);
        }

        serving_status(message.status)
    }
}

#[cfg(test)]
mod tests {
    use super::health_service::{health_check_response::ServingStatus, serving_status};

    #[test]
    fn serving_status_ok() {
        assert!(serving_status(ServingStatus::Serving as i32).is_ok());
    }

    #[test]
    fn serving_status_err() {
        assert!(serving_status(ServingStatus::Unknown as i32).is_err());
        assert!(serving_status(ServingStatus::NotServing as i32).is_err());
        assert!(serving_status(ServingStatus::ServiceUnknown as i32).is_err());
        assert!(serving_status(99_i32).is_err());
    }
}
