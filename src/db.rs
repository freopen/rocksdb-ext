use std::path::Path;

pub struct DB {
    pub rocksdb: rocksdb::DB,
}

#[derive(Default)]
pub struct DBConfig {
    pub opts: rocksdb::Options,
    pub cfs: Vec<rocksdb::ColumnFamilyDescriptor>,
}

impl DBConfig {
    pub fn open(self, path: &Path) -> Result<DB, rocksdb::Error> {
        rocksdb::DB::open_cf_descriptors(&self.opts, path, self.cfs.into_iter())
            .map(|db| DB { rocksdb: db })
    }
}

#[cfg(test)]
mod test {
    use crate::db::*;
    use anyhow::Result;

    #[test]
    fn open_database() -> Result<()> {
        let dir = assert_fs::TempDir::new()?.into_persistent();
        dbg!(&dir);
        let mut config = DBConfig::default();
        config.opts.create_if_missing(true);
        config.open(dir.path())?;
        Ok(())
    }
}
