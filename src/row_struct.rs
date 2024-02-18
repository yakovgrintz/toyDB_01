use crate::db_type::DbType;

pub(crate) struct TableRow {
    values: Vec<DbType>,
}
impl Drop for TableRow {
    fn drop(&mut self) {
        todo!()
    }
}
impl TableRow {
    pub(crate) fn new(values: Vec<DbType>) -> Self {
        TableRow { values }
    }
    pub(crate) fn get_values(self) -> Vec<DbType> {
        self.values
    }
}
