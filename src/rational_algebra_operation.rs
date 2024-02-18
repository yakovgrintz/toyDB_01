use crate::db_type::DbType;
use crate::row_struct::TableRow;
use crate::ManipulateTable;
use crate::query_result::QueryResult;

fn selection<T, F>(table: T, condition: F) -> T
where
    T: ManipulateTable,
{
    todo!()
}
fn projection<T>(table: T, column: Vec<str>) -> T
where
    T: ManipulateTable,
{
    assert!(table.get_column_names().len() >= column.len());
    let column_names: Vec<String> = table.get_column_names();
    let indices: Vec<usize> = column
        .iter()
        .filter_map(|name| column_names.iter().position(|c| c == name))
        .collect();
    let result: Vec<TableRow> = table
        .get_data()
        .iter()
        .map(|row| {
            let projected_row: Vec<DbType> = indices
                .iter()
                .filter_map(|&index| row.get_values().get(index))
                .cloned()
                .collect();
            TableRow::new(projected_row)
        })
        .collect();
    let result_column_names: Vec<String> = indices
    .iter()
    .map(|&index| column_names[index].clone())
    .collect();
    QueryResult{ data:result, column_names:result_column_names }
}
