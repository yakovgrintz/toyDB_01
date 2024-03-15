use crate::db_type::DbType;
use crate::query_result::QueryResult;
use crate::rational_algebra::helper_functions::find_indexes;
use crate::table_row::TableRow;
use crate::ManipulateTable;

pub(crate) fn projection<T>(table: &T, columns: &[String]) -> QueryResult
where
    T: ManipulateTable,
{
    assert!(table.get_column_names().len() >= columns.len());
    let column_names: &Vec<String> = table.get_column_names();
    let indexes = find_indexes(table, columns);
    let result: Vec<TableRow> = table
        .get_data()
        .iter()
        .map(|row| {
            let projected_row: Vec<DbType> = indexes
                .iter()
                .filter_map(|&index| row.get_values().get(index))
                .cloned()
                .collect();
            TableRow::new(projected_row)
        })
        .collect();
    let result_column_names: Vec<String> = indexes
        .iter()
        .map(|&index| column_names[index].clone())
        .collect();
    QueryResult::new(result, result_column_names)
}
