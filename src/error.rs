//! Ubiquitous types for error management.

use std::error::Error as StdError;
use std::io::Error as IoError;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use std::str::ParseBoolError;

use quick_xml::Error as XmlError;
use thiserror::Error;
use ureq::Error as UreqError;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
/// An error returned by the Power User Gateway REST API.
pub enum ApiError {
    #[error("bad request: {0}")]
    /// Request is improperly formed.
    BadRequest(String),
    #[error("not found: {0}")]
    /// The input record was not found.
    NotFound(String),
    #[error("not allowed: {0}")]
    /// Request not allowed.
    NotAllowed(String),
    #[error("timeout: {0}")]
    /// The request timed out, from server overload or too broad a request.
    Timeout(String),
    #[error("server busy: {0}")]
    /// Too many requests or server is busy, retry later.
    ServerBusy(String),
    #[error("unimplemented!(): {0}")]
    /// The requested operation has not (yet) been implemented by the server.
    Unimplemented(String),
    #[error("server error: {0}")]
    /// Some problem on the server side (such as a database server down).
    ServerError(String),
    #[error("unknown error: {0}")]
    /// An unknown error occurred
    Unknown(String),
}

impl From<crate::model::rest::Fault> for ApiError {
    fn from(fault: crate::model::rest::Fault) -> Self {
        match fault.code.as_str() {
            "PUGREST.BadRequest" => ApiError::BadRequest(fault.message),
            "PUGREST.NotFound" => ApiError::NotFound(fault.message),
            "PUGREST.NotAllowed" => ApiError::NotAllowed(fault.message),
            "PUGREST.Timeout" => ApiError::Timeout(fault.message),
            "PUGREST.ServerBusy" => ApiError::ServerBusy(fault.message),
            "PUGREST.Unimplemented" => ApiError::Unimplemented(fault.message),
            "PUGREST.ServerError" => ApiError::ServerError(fault.message),
            _ => ApiError::Unknown(fault.message),
        }
    }
}

// ---------------------------------------------------------------------------

#[derive(Debug, Error, Clone, PartialEq, Eq)]
/// An error raised by a `FromStr` implementor.
pub enum ParseError {
    #[error(transparent)]
    Int(#[from] ParseIntError),
    #[error(transparent)]
    Float(#[from] ParseFloatError),
}

// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
/// The main error type for the [`pubchem`] crate.
///
/// [`pubchem`]: ../index.html
pub enum Error {
    #[error(transparent)]
    /// The PubChem API responded with an error.
    Api(#[from] ApiError),
    #[error(transparent)]
    /// The HTTP client encountered an error.
    Request(#[from] UreqError),
    #[error(transparent)]
    /// The XML parser encountered an error.
    ///
    /// *Any error from the underlying reader will be wrapped in the
    /// [`XmlError::Io`] variant.*
    ///
    /// [`XmlError::Io`]: https://docs.rs/quick-xml/latest/quick_xml/enum.Error.html#variant.Io
    Xml(#[from] XmlError),
    #[error(transparent)]
    /// A parser returned an error.
    Parse(#[from] ParseError),
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::from(XmlError::Io(e))
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::Parse(ParseError::Int(e))
    }
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Self::Parse(ParseError::Float(e))
    }
}

/// The main result type for the [`pubchem`] crate.
///
/// [`pubchem`]: ../index.html
pub type Result<T> = std::result::Result<T, Error>;
