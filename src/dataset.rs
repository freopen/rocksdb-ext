pub mod collection;

use crate::{DBConfig, DB};

pub trait Dataset {}

pub trait DatasetConfig {
    type DatasetType: Dataset;

    fn update_db_config(&self, db_config: &mut DBConfig);
    fn open(&self, db: &DB) -> Self::DatasetType;
}
