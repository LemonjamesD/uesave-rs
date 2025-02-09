#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("bad magic value (GVAS)")]
    BadMagic(),
    #[error("unknown property type: {0}")]
    UnknownPropertyType(String),
    #[error("unknown PropertyMeta type: {0}")]
    UnknownPropertyMeta(String),
    #[error("unknown vec type: {0}")]
    UnknownVecType(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(&'static str),
}
