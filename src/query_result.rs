use crate::row_struct::TableRow;
use crate::ManipulateTable;

pub(crate) struct QueryResult {
    data: Vec<TableRow>,
    column_names: Vec<String>,
}

impl ManipulateTable for QueryResult {
    fn get_column_names(&self)->Vec<String> {
        self.column_names.clone()
    }

    fn get_data(&self)->Vec<TableRow> {
        self.data.clone()
    }
}
