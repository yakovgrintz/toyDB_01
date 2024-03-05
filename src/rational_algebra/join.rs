use crate::query_result::QueryResult;
use crate::table_row::TableRow;
use crate::ManipulateTable;
use std::error::Error;
use std::fmt;

fn inner_join<T>(table1: &T, table2: &T, by: &str) -> Result<QueryResult, JoinError>
where
    T: ManipulateTable,
{
    let index_1 = table1.get_column_names().iter().position(|name| name == by);
    let index_2 = table2.get_column_names().iter().position(|name| name == by);
    if index_1.is_none() || index_2.is_none() {
        return Err(JoinError {
            message: format!("Column '{}' not found in one of the tables", by),
        });
    }
    let index_1 = index_1.unwrap();
    let index_2 = index_2.unwrap();
    let data_1 = table1.get_data();
    let data_2 = table2.get_data();
    let mut result: Vec<TableRow> = Vec::new();
    let mut column_names_result: Vec<String> = Vec::new();
    for row1 in data_1 {
        for row2 in data_2 {
            if let Some(value1) = row1.get_values().get(index_1) {
                if let Some(value2) = row2.get_values().get(index_2) {
                    if value1 == value2 {
                        result.push(create_row(&row1, &row2, index_2))
                    }
                }
            }
        }
    }
    column_names_result.extend_from_slice(table1.get_column_names());
    for (index, value) in table2.get_column_names().iter().enumerate() {
        if index != index_2 {
            column_names_result.push(value.clone());
        }
    }
    Ok(QueryResult::new(result, column_names_result))
}
fn create_row(row1: &TableRow, row2: &TableRow, index2: usize) -> TableRow {
    let mut values = Vec::new();
    values.extend_from_slice(row1.get_values());
    for (index, value) in row2.get_values().iter().enumerate() {
        if index != index2 {
            values.push(value.clone());
        }
    }
    TableRow::new(values)
}
#[derive(Debug)]
struct JoinError {
    message: String,
}

impl Error for JoinError {}

impl fmt::Display for JoinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Join Error: {}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use crate::db_type::DbType;
    use super::*;
    struct MockTable {
        column_names: Vec<String>,
        data: Vec<TableRow>,
    }

    impl MockTable {
        fn new(column_names: Vec<String>, data: Vec<TableRow>) -> Self {
            MockTable { column_names, data }
        }
    }

    impl ManipulateTable for MockTable {
        fn get_column_names(&self) -> &Vec<String> {
            &self.column_names
        }

        fn get_data(&self) -> &Vec<TableRow> {
            &self.data
        }
    }

    fn set_up_table1() -> MockTable {
        MockTable {
            column_names: vec!["id".to_string(), "name".to_string()],
            data: vec![
                TableRow::new(vec![DbType::Int(1), DbType::Text("Alice".to_string())]),
                TableRow::new(vec![DbType::Int(2), DbType::Text("Bob".to_string())]),
            ],
        }
    }

    fn set_up_table2() -> MockTable {
        MockTable {
            column_names: vec!["id".to_string(), "age".to_string()],
            data: vec![
                TableRow::new(vec![DbType::Int(1), DbType::Int(30)]),
                TableRow::new(vec![DbType::Int(2), DbType::Int(25)]),
            ],
        }
    }

    #[test]
    fn test_inner_join_success() {
        let table1 = set_up_table1();
        let table2 = set_up_table2();

        let result = inner_join(&table1, &table2, "id").unwrap();
        assert_eq!(result.get_data().len(), 2); // This assumes QueryResult has a method `get_rows`
    }

    #[test]
    fn test_inner_join_column_not_found() {
        let table1 = set_up_table1();
        let table2 = MockTable {
            column_names: vec!["user_id".to_string(), "age".to_string()],
            data: vec![
                TableRow::new(vec![DbType::Int(3), DbType::Int(40)]),
                TableRow::new(vec![DbType::Int(4), DbType::Int(22)]),
            ],
        };

        let result = inner_join(&table1, &table2, "id");
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.message, "Column 'id' not found in one of the tables");
        }
    }
}
