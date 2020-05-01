use crate::{
    hash::{Hash, HashPrimitive},
    DataPoint, DataPointSlice, Error, Result, VecHash,
};
use fnv::{FnvHashSet as HashSet, FnvHashSet};
use serde::{de::DeserializeOwned, Serialize};

/// Bucket contains indexes to VecStore
pub type Bucket = HashSet<u32>;

/// Hashtable consisting of `L` Hash tables.
pub trait HashTables {
    fn new(n_hash_tables: usize, only_index_storage: bool, db_path: &str) -> Result<Box<Self>>;

    /// # Arguments
    ///
    /// * `hash` - hashed vector.
    /// * `d` - Vector to store in the buckets.
    /// * `hash_table` - Number of the hash_table to store the vector. Ranging from 0 to L.
    fn put(&mut self, hash: Hash, d: &DataPointSlice, hash_table: usize) -> Result<u32>;

    fn delete(&mut self, _hash: &Hash, _d: &DataPointSlice, _hash_table: usize) -> Result<()> {
        Err(Error::NotImplemented)
    }

    fn update_by_idx(
        &mut self,
        _old_hash: &Hash,
        _new_hash: Hash,
        _idx: u32,
        _hash_table: usize,
    ) -> Result<()> {
        Err(Error::NotImplemented)
    }

    /// Query the whole bucket
    fn query_bucket(&self, hash: &Hash, hash_table: usize) -> Result<Bucket>;

    fn idx_to_datapoint(&self, _idx: u32) -> Result<&DataPoint> {
        Err(Error::NotImplemented)
    }

    fn increase_storage(&mut self, _size: usize) {}

    fn describe(&self) -> Result<String> {
        Err(Error::NotImplemented)
    }

    // Should fail if hashers already stored.
    fn store_hashers<N, H: VecHash<N> + Serialize>(&mut self, _hashers: &[H]) -> Result<()> {
        Ok(())
    }

    // If store_hashers fails, load_hasher can be executed
    fn load_hashers<N, H: VecHash<N> + DeserializeOwned>(&self) -> Result<Vec<H>> {
        // just chose an error to make a default trait implementation
        Err(Error::NotImplemented)
    }

    fn get_unique_hash_int(&self) -> FnvHashSet<HashPrimitive>;
}
