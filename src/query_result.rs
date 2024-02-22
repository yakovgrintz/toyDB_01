use crate::table_row::TableRow;
use crate::ManipulateTable;

pub(crate) struct QueryResult {
    data: Vec<TableRow>,
    column_names: Vec<String>,
}
impl QueryResult {
    pub(crate) fn new(data: Vec<TableRow>, column: Vec<String>) -> QueryResult {
        QueryResult {
            data,
            column_names: column,
        }
    }
}

impl ManipulateTable for QueryResult {
    fn get_column_names(&self) -> &Vec<String> {
        &self.column_names
    }

    fn get_data(&self) -> &Vec<TableRow> {
        &self.data
    }
}
