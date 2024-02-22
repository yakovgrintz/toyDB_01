use crate::db_type::DbType;
use crate::query_result::QueryResult;
use crate::table_row::TableRow;
use crate::ManipulateTable;

// fn selection<T, F>(table: T, condition: F) -> T
// where
//     T: ManipulateTable,
// {
//
//         let tokens = condition.split_whitespace().collect();
//         let field: &str = tokens[0];
//         let operator = tokens[1];
//         let value = tokens[2];
//         let index = table.get_column_names().iter().position(|c| c == field);
//         let result: Vec<TableRow> = table
//             .get_data()
//             .iter()
//             .map(|row| {
//                 let row_value = row.get_values().get(index).expect("Index out of bounds");
//                 let matches = match operator {
//                     "<" => row.get_values().get(index) < value,
//                     ">" => row.get_values().get(index) > value,
//                     "=" => row.get_values().get(index) == value,
//                     "<=>" => row.get_values().get(index) <= value,
//                     ">=" => row.get_values().get(index) >= value,
//                     "!=" => row.get_values().get(index) != value,
//                 };
//                 if matches {
//                     Some(row.clone())
//                 } else {
//                     None
//                 }
//             })
//             .collect();
//         QueryResult {
//             data: vec![],
//             column_names: vec![],
//         }
//
// }
fn projection<T>(table: T, column: Vec<String>) -> QueryResult
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
    QueryResult::new(result, result_column_names)
}
