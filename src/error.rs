pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid configuration format: {0}")]
    InvalidConfiguration(#[from] toml::de::Error),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}
