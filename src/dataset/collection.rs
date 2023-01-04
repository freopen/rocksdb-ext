use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    dataset::{Dataset, DatasetConfig},
    Error, Result, DB,
};

#[derive(Clone)]
pub struct Collection<
    KeyType: Serialize + DeserializeOwned,
    ValueType: Serialize + DeserializeOwned,
> {
    db: DB,
    cf_name: String,
    _phantom: PhantomData<(KeyType, ValueType)>,
}

impl<KeyType: Serialize + DeserializeOwned, ValueType: Serialize + DeserializeOwned> Dataset
    for Collection<KeyType, ValueType>
{
}

impl<KeyType: Serialize + DeserializeOwned, ValueType: Serialize + DeserializeOwned>
    Collection<KeyType, ValueType>
{
    pub fn get(&self, key: &KeyType) -> Result<Option<ValueType>> {
        let cf = self
            .db
            .rocksdb
            .cf_handle(&self.cf_name)
            .ok_or_else(|| Error::CFNotFound(self.cf_name.to_owned()))?;
        if let Some(value_bytes) = self
            .db
            .rocksdb
            .get_pinned_cf(cf, storekey::serialize(key)?)?
        {
            Ok(rmp_serde::from_slice(&value_bytes)?)
        } else {
            Ok(None)
        }
    }
    pub fn put(&self, key: &KeyType, value: &ValueType) -> Result<()> {
        let cf = self
            .db
            .rocksdb
            .cf_handle(&self.cf_name)
            .ok_or_else(|| Error::CFNotFound(self.cf_name.to_owned()))?;
        self.db
            .rocksdb
            .put_cf(cf, storekey::serialize(key)?, rmp_serde::to_vec(value)?)?;
        Ok(())
    }
    pub fn delete(&self, key: &KeyType) -> Result<()> {
        let cf = self
            .db
            .rocksdb
            .cf_handle(&self.cf_name)
            .ok_or_else(|| Error::CFNotFound(self.cf_name.to_owned()))?;
        self.db.rocksdb.delete_cf(cf, storekey::serialize(key)?)?;
        Ok(())
    }
}

pub struct CollectionConfig<
    KeyType: Serialize + DeserializeOwned,
    ValueType: Serialize + DeserializeOwned,
> {
    pub name: String,
    pub cf_options: rocksdb::Options,
    _phantom: PhantomData<(KeyType, ValueType)>,
}

impl<KeyType: Serialize + DeserializeOwned, ValueType: Serialize + DeserializeOwned> DatasetConfig
    for CollectionConfig<KeyType, ValueType>
{
    type DatasetType = Collection<KeyType, ValueType>;

    fn update_db_config(&self, db_config: &mut crate::DBConfig) {
        db_config
            .cfs
            .push((self.name.clone(), self.cf_options.clone()));
    }

    fn open(&self, db: &DB) -> Self::DatasetType {
        Self::DatasetType {
            db: db.clone(),
            cf_name: self.name.clone(),
            _phantom: PhantomData::default(),
        }
    }
}

impl<KeyType: Serialize + DeserializeOwned, ValueType: Serialize + DeserializeOwned>
    CollectionConfig<KeyType, ValueType>
{
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            cf_options: rocksdb::Options::default(),
            _phantom: PhantomData::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{dataset::DatasetConfig, *};
    use anyhow::Result;

    #[test]
    fn basic() -> Result<()> {
        let dir = assert_fs::TempDir::new()?;
        let mut db_config = DBConfig::default();
        db_config.opts.create_if_missing(true);
        db_config.opts.create_missing_column_families(true);
        let collection_config = CollectionConfig::<i32, i32>::new("test");
        collection_config.update_db_config(&mut db_config);
        {
            let db = db_config.open(dir.path())?;
            let collection = collection_config.open(&db);
            assert_eq!(collection.get(&1)?, None);
            collection.put(&1, &2)?;
            assert_eq!(collection.get(&1)?, Some(2));
        }
        {
            let db = db_config.open(dir.path())?;
            let collection = collection_config.open(&db);
            assert_eq!(collection.get(&1)?, Some(2));
            collection.delete(&1)?;
            assert_eq!(collection.get(&1)?, None);
        }
        Ok(())
    }

    #[test]
    fn singleton() -> Result<()> {
        let dir = assert_fs::TempDir::new()?;
        let mut db_config = DBConfig::default();
        db_config.opts.create_if_missing(true);
        db_config.opts.create_missing_column_families(true);
        let collection_config = CollectionConfig::<(), i32>::new("test");
        collection_config.update_db_config(&mut db_config);
        let db = db_config.open(dir.path())?;
        let collection = collection_config.open(&db);
        assert_eq!(collection.get(&())?, None);
        collection.put(&(), &1)?;
        assert_eq!(collection.get(&())?, Some(1));
        Ok(())
    }
    #[test]
    fn tuple() -> Result<()> {
        let dir = assert_fs::TempDir::new()?;
        let mut db_config = DBConfig::default();
        db_config.opts.create_if_missing(true);
        db_config.opts.create_missing_column_families(true);
        let collection_config = CollectionConfig::<(i32, i32), i32>::new("test");
        collection_config.update_db_config(&mut db_config);
        let db = db_config.open(dir.path())?;
        let collection = collection_config.open(&db);
        assert_eq!(collection.get(&(1, 2))?, None);
        collection.put(&(1, 2), &1)?;
        assert_eq!(collection.get(&(1, 2))?, Some(1));
        Ok(())
    }
}
