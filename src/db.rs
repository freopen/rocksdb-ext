use std::{path::Path, sync::Arc};

#[derive(Clone)]
pub struct DB {
    pub rocksdb: Arc<rocksdb::DB>,
}

#[derive(Default)]
pub struct DBConfig {
    pub opts: rocksdb::Options,
    pub cfs: Vec<(String, rocksdb::Options)>,
}

impl DBConfig {
    pub fn open(&self, path: &Path) -> Result<DB, rocksdb::Error> {
        rocksdb::DB::open_cf_descriptors(
            &self.opts,
            path,
            self.cfs.iter().map(|(name, opts)| {
                rocksdb::ColumnFamilyDescriptor::new(name.clone(), opts.clone())
            }),
        )
        .map(|db| DB {
            rocksdb: Arc::new(db),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use anyhow::Result;

    #[test]
    fn open_database() -> Result<()> {
        let dir = assert_fs::TempDir::new()?;
        let mut config = DBConfig::default();
        config.opts.create_if_missing(true);
        config.open(dir.path())?;
        Ok(())
    }
}
