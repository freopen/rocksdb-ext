#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("RocksDB internal error")]
    RocksdbError(#[from] rocksdb::Error),
    #[error("Column family {0} not found")]
    CFNotFound(String),
    #[error("Key encode error")]
    KeyEncodeError(#[from] storekey::encode::Error),
    #[error("Value encode error")]
    ValueEncodeError(#[from] rmp_serde::encode::Error),
    #[error("Value decode error")]
    ValueDecodeError(#[from] rmp_serde::decode::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
