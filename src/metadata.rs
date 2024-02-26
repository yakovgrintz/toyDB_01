use crate::bloom_filter::BloomFilter;
use crate::db_type::DbType;

pub(crate) struct MetaData {
    primary_key: Vec<usize>,
    pk_filter: BloomFilter<Vec<DbType>>,
    table_capacity: usize,
}
impl MetaData {
    pub(crate) fn new(table_capacity: usize, error_percent: f32) -> Self {
        MetaData {
            primary_key: vec![],
            pk_filter: BloomFilter::new(table_capacity as f32, error_percent),
            table_capacity,
        }
    }
    pub(crate) fn set_pk(&mut self, indexes: Vec<usize>) {
        self.primary_key = indexes.clone();
    }
    pub(crate) fn get_pk(&self) -> &Vec<usize> {
        &self.primary_key
    }
    pub(crate) fn get_table_capacity(&self) -> usize {
        self.table_capacity
    }
    pub(crate) fn get_filter(&mut self) -> &mut BloomFilter<Vec<DbType>> {
        &mut self.pk_filter
    }
}
