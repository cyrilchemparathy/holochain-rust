//! Connection Error struct and TransportResult type

/// represents an error generated by a connection instance
#[derive(Debug, PartialEq, Clone)]
pub struct TransportError(pub String);

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TransportError {
    fn description(&self) -> &str {
        &self.0
    }
    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

impl From<Vec<TransportError>> for TransportError {
    fn from(errors: Vec<TransportError>) -> Self {
        Self(format!("{:?}", errors))
    }
}

impl From<url::ParseError> for TransportError {
    fn from(error: url::ParseError) -> Self {
        Self(format!("{:?}", error))
    }
}

impl From<std::io::Error> for TransportError {
    fn from(error: std::io::Error) -> Self {
        Self(format!("{:?}", error))
    }
}

impl From<tungstenite::Error> for TransportError {
    fn from(error: tungstenite::Error) -> Self {
        Self(format!("{:?}", error))
    }
}

impl<T: std::io::Read + std::io::Write + std::fmt::Debug> From<native_tls::HandshakeError<T>>
    for TransportError
{
    fn from(error: native_tls::HandshakeError<T>) -> Self {
        Self(format!("{:?}", error))
    }
}

impl<T: std::io::Read + std::io::Write + std::fmt::Debug>
    From<tungstenite::HandshakeError<tungstenite::ClientHandshake<T>>> for TransportError
{
    fn from(error: tungstenite::HandshakeError<tungstenite::ClientHandshake<T>>) -> Self {
        Self(format!("{:?}", error))
    }
}

/// a result object whos error is a TransportError instance
pub type TransportResult<T> = Result<T, TransportError>;
