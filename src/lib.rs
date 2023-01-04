pub mod dataset;
pub mod db;
pub mod error;

pub use crate::{
    dataset::{
        collection::{Collection, CollectionConfig},
        DatasetConfig,
    },
    db::{DBConfig, DB},
    error::{Error, Result},
};
pub use rocksdb;
