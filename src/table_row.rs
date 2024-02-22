use crate::db_type::DbType;
#[derive(Clone)]
pub(crate) struct TableRow {
    values: Vec<DbType>,
}

impl TableRow {
    pub(crate) fn new(values: Vec<DbType>) -> Self {
        TableRow { values }
    }
    pub(crate) fn get_values(&self) -> &Vec<DbType> {
        &self.values
    }
}
