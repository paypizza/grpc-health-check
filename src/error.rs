use std::{error, fmt};

type GenericError = Box<dyn error::Error + Send + Sync>;

#[derive(Debug)]
pub enum ErrorKind {
    /// Invalid configuration.
    InvalidConfig,
    /// crate::std::io::Error.
    Io,
    /// crate::http.
    Http,
    /// crate::tonic::transport::Error.
    Transport,
}

struct ErrorImpl {
    kind: ErrorKind,
    source: Option<GenericError>,
}

pub struct Error {
    inner: ErrorImpl,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self {
            inner: ErrorImpl { kind, source: None },
        }
    }

    pub(crate) fn with(mut self, source: impl Into<GenericError>) -> Self {
        self.inner.source = Some(source.into());
        self
    }

    pub(crate) fn from_http(source: impl Into<GenericError>) -> Self {
        Error::new(ErrorKind::Http).with(source)
    }

    pub(crate) fn from_io(source: impl Into<GenericError>) -> Self {
        Error::new(ErrorKind::Io).with(source)
    }

    pub(crate) fn from_transport(source: impl Into<GenericError>) -> Self {
        Error::new(ErrorKind::Transport).with(source)
    }

    fn description(&self) -> &str {
        match &self.inner.kind {
            ErrorKind::InvalidConfig => "config error",
            ErrorKind::Io => "io error",
            ErrorKind::Http => "http error",
            ErrorKind::Transport => "transport error",
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_tuple("grpc_health_check::Error");
        f.field(&self.inner.kind);

        if let Some(source) = &self.inner.source {
            f.field(source);
        }

        f.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source) = &self.inner.source {
            write!(f, "{}: {}", self.description(), source)
        } else {
            f.write_str(self.description())
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.inner
            .source
            .as_ref()
            .map(|source| &**source as &(dyn error::Error + 'static))
    }
}
