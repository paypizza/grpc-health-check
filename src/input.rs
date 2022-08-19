use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

/// Options.
#[derive(Serialize, StructOpt, Debug)]
#[structopt(name = "grpc-health-check", setting = structopt::clap::AppSettings::TrailingVarArg)]
pub struct Opts {
    /// Address to the gRPC server.
    #[structopt(long, default_value = "localhost")]
    pub address: String,

    /// File path to the YAML configuration file.
    #[structopt(short, long, parse(from_os_str))]
    pub config: Option<PathBuf>,

    /// Port to the gRPC server.
    #[structopt(long)]
    pub port: Option<u16>,

    /// Timeout for establishing connection in milliseconds (ms)
    #[structopt(long, default_value = "1500")]
    pub rpc_timeout: u64,

    /// Name of the service to check. An empty string is the default value, according to the gRPC Health Check Protocol specification.
    #[structopt(long)]
    pub service_name: Option<String>,

    /// Enable RPC streaming mode (default *false*).
    #[structopt(long)]
    pub stream: Option<bool>,

    /// File path to the CA certificate against which to verify the server's certificate.
    #[structopt(long, parse(from_os_str))]
    pub tls_ca_cert: Option<PathBuf>,

    /// File path to the client's certificate.
    #[structopt(long, parse(from_os_str))]
    pub tls_client_cert: Option<PathBuf>,

    /// File path to the client's private key.
    #[structopt(long, parse(from_os_str))]
    pub tls_client_key: Option<PathBuf>,

    /// Name of the domain against which to verify the server's certificate.
    #[structopt(long)]
    pub tls_domain_name: Option<String>,

    /// Name of the user agent.
    #[structopt(long)]
    pub user_agent: Option<String>,

    /// Enable verbose mode.
    #[structopt(long)]
    pub verbose: bool,
}

#[cfg(not(tarpaulin_include))]
impl Opts {
    pub fn new() -> Self {
        Opts::from_args()
    }

    pub fn merge(&mut self, config: &Config) {
        if let Some(value) = &config.address {
            self.address = value.clone();
        }

        if let Some(value) = &config.port {
            self.port = Option::from(*value);
        }

        if let Some(value) = &config.rpc_timeout {
            self.rpc_timeout = *value;
        }

        if let Some(value) = &config.service_name {
            self.service_name = Option::from(value.clone());
        }

        if let Some(value) = &config.stream {
            self.stream = Option::from(*value);
        }

        if let Some(value) = &config.tls_ca_cert {
            self.tls_ca_cert = Option::from(value.clone());
        }

        if let Some(value) = &config.tls_client_cert {
            self.tls_client_cert = Option::from(value.clone());
        }

        if let Some(value) = &config.tls_client_key {
            self.tls_client_key = Option::from(value.clone());
        }

        if let Some(value) = &config.tls_domain_name {
            self.tls_domain_name = Option::from(value.clone());
        }

        if let Some(value) = &config.user_agent {
            self.user_agent = Option::from(value.clone());
        }
    }
}

/// Configuration.
#[cfg(not(tarpaulin_include))]
#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: Option<String>,

    pub port: Option<u16>,

    pub rpc_timeout: Option<u64>,

    pub service_name: Option<String>,

    pub stream: Option<bool>,

    pub tls_ca_cert: Option<PathBuf>,

    pub tls_client_cert: Option<PathBuf>,

    pub tls_client_key: Option<PathBuf>,

    pub tls_domain_name: Option<String>,

    pub user_agent: Option<String>,
}

#[cfg(not(tarpaulin_include))]
impl Config {
    /// Parses the YAML string it into a config struct.
    pub fn parse_yaml(content: &str) -> serde_yaml::Result<Config> {
        serde_yaml::from_str(content)
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Opts};
    use std::path::PathBuf;

    impl Opts {
        pub fn default() -> Self {
            Self {
                address: "foo".to_string(),
                config: None,
                port: Option::from(5200),
                rpc_timeout: 0,
                service_name: Option::from("foo".to_owned()),
                stream: None,
                tls_ca_cert: None,
                tls_client_cert: None,
                tls_client_key: None,
                tls_domain_name: None,
                user_agent: None,
                verbose: false,
            }
        }
    }

    impl Config {
        fn default() -> Self {
            Self {
                address: None,
                port: None,
                rpc_timeout: None,
                service_name: None,
                stream: None,
                tls_ca_cert: None,
                tls_client_cert: None,
                tls_client_key: None,
                tls_domain_name: None,
                user_agent: None,
            }
        }
    }

    #[test]
    fn opts_merge() {
        let mut opts = Opts::default();
        let mut config = Config::default();
        config.address = Option::from("127.0.0.1".to_owned());
        config.port = Option::from(5400);
        config.rpc_timeout = Option::from(3000);
        config.service_name = Option::from("bar".to_owned());
        config.stream = Some(true);
        config.tls_ca_cert = Option::from(PathBuf::from("tests/data/ca.pem".to_owned()));
        config.tls_client_cert = Option::from(PathBuf::from("tests/data/client.pem".to_owned()));
        config.tls_client_key = Option::from(PathBuf::from("tests/data/client.key".to_owned()));
        config.tls_domain_name = Option::from("localhost.localdomain".to_owned());
        config.user_agent = Option::from("Health 1.1".to_owned());

        opts.merge(&config);

        assert_eq!(opts.address, "127.0.0.1");
        assert_eq!(opts.port, Some(5400));
        assert_eq!(opts.rpc_timeout, 3000);
        assert_eq!(opts.service_name, Some("bar".to_owned()));
        assert_eq!(opts.stream, Some(true));
        assert_eq!(
            opts.tls_ca_cert,
            Some(PathBuf::from("tests/data/ca.pem".to_owned()))
        );
        assert_eq!(
            opts.tls_client_cert,
            Some(PathBuf::from("tests/data/client.pem".to_owned()))
        );
        assert_eq!(
            opts.tls_client_key,
            Some(PathBuf::from("tests/data/client.key".to_owned()))
        );
        assert_eq!(
            opts.tls_domain_name,
            Some("localhost.localdomain".to_owned())
        );
        assert_eq!(opts.user_agent, Some("Health 1.1".to_owned()));
    }

    #[test]
    fn opts_merge_empty() {
        let mut opts = Opts::default();
        let config = Config::default();

        opts.merge(&config);

        assert_eq!(opts.address, opts.address);
        assert_eq!(opts.port, opts.port);
        assert_eq!(opts.rpc_timeout, opts.rpc_timeout);
        assert_eq!(opts.service_name, opts.service_name);
        assert_eq!(opts.stream, opts.stream);
        assert_eq!(opts.tls_ca_cert, opts.tls_ca_cert);
        assert_eq!(opts.tls_client_cert, opts.tls_client_cert);
        assert_eq!(opts.tls_client_key, opts.tls_client_key);
        assert_eq!(opts.tls_domain_name, opts.tls_domain_name);
        assert_eq!(opts.user_agent, opts.user_agent);
    }
}
