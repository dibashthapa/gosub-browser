//! Error results that can be returned from the engine
use thiserror::Error;

/// Parser error that defines an error (message) on the given position
#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    /// Parse error message
    pub message: String,
    /// Line number (1-based) of the error
    pub line: usize,
    // Column (1-based) on line of the error
    pub col: usize,
    // Position (0-based) of the error in the input stream
    pub offset: usize,
}

/// Serious errors and errors from third-party libraries
#[derive(Error, Debug)]
pub enum Error {
    #[error("config error: {0}")]
    Config(String),

    #[error("ureq error")]
    Request(#[from] Box<ureq::Error>),

    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("utf8 conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("json parsing error: {0}")]
    JsonSerde(#[from] serde_json::Error),

    #[error("sqlite error: {0}")]
    Sqlite(#[from] sqlite::Error),

    #[error("test error: {0}")]
    Test(String),

    #[error("document task error: {0}")]
    DocumentTask(String),

    #[error("query: generic error: {0}")]
    Query(String),

    #[error("dns: generic error: {0}")]
    DnsGeneric(String),

    #[error("dns: no ipv6 address found")]
    DnsNoIpv6Found,

    #[error("dns: no ipv4 address found")]
    DnsNoIpv4Found,

    #[error("dns: no ip address found")]
    DnsNoIpAddressFound,

    #[error("dns: domain not found")]
    DnsDomainNotFound,
}

/// Result that can be returned which holds either T or an Error
pub type Result<T> = std::result::Result<T, Error>;
