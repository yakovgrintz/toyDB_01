#![warn(clippy::pedantic)]
#![warn(clippy::complexity)]

use crate::table_row::TableRow;

mod bloom_filter;
mod condition;
mod db_type;
mod metadata;
mod query_result;
mod rational_algebra;
mod schema_struct;
mod table_row;
mod table_struct;
mod test_to_rational_algebra_operation;

pub(crate) trait ManipulateTable {
    fn get_column_names(&self) -> &Vec<String>;
    fn get_data(&self) -> &Vec<TableRow>;
}
