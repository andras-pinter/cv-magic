pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid template format: {0}")]
    InvalidTemplate(#[from] tera::Error),
    #[error("invalid configuration format: {0}")]
    InvalidConfiguration(#[from] toml::de::Error),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("invalid IP address: {0}")]
    InvalidIpAddress(#[from] std::net::AddrParseError),
}
