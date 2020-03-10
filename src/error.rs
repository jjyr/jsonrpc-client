use jsonrpc_core::Error as JsonRPCError;
use reqwest::Error as ConnectionError;
use serde_json::Error as SerdeError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Jsonrpc(JsonRPCError),
    Serialize(SerdeError),
    Connection(ConnectionError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Jsonrpc(err) => write!(f, "{}", err)?,
            Self::Serialize(err) => write!(f, "{}", err)?,
            Self::Connection(err) => write!(f, "{}", err)?,
        }
        Ok(())
    }
}

impl ::std::error::Error for Error {}

impl From<JsonRPCError> for Error {
    fn from(err: JsonRPCError) -> Self {
        Error::Jsonrpc(err)
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Self {
        Error::Serialize(err)
    }
}

impl From<ConnectionError> for Error {
    fn from(err: ConnectionError) -> Self {
        Error::Connection(err)
    }
}
