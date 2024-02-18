use crate::row_struct::TableRow;

mod db_type;
mod metadata;
mod query_result;
mod rational_algebra_operation;
mod row_struct;
mod schema_struct;
mod table_struct;
pub trait ManipulateTable {
    fn get_column_names(&self) -> Vec<String>;
    fn get_data(&self) -> Vec<TableRow>;

}
