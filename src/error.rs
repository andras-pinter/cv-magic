use rocket::http::Status;
use rocket::response::status::Custom;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid configuration format: {0}")]
    InvalidConfiguration(#[from] toml::de::Error),
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("renderer error: {0}")]
    RendererError(#[from] tera::Error),
    #[error("invalid format")]
    InvalidFormat,
    #[error("internal render error")]
    InternalRendererError,
}

impl From<Error> for Custom<String> {
    fn from(err: Error) -> Self {
        Custom(Status::InternalServerError, err.to_string())
    }
}
