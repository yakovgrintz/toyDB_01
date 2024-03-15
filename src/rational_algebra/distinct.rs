use crate::db_type::DbType;
use crate::query_result::QueryResult;
use crate::rational_algebra::helper_functions::find_indexes;
use crate::ManipulateTable;
use std::collections::HashSet;

fn distinct<T>(table: &T, columns: &[String]) -> QueryResult
where
    T: ManipulateTable,
{
    let indexes = find_indexes(table, columns);
    let mut seen: HashSet<Vec<DbType>> = HashSet::new();
    let result = table
        .get_data()
        .iter()
        .filter(|row| {
            let values = indexes
                .iter()
                .filter_map(|&index| row.get_values().get(index).cloned())
                .collect();
            seen.insert(values)
        })
        .cloned()
        .collect();
    QueryResult::new(result, table.get_column_names().clone())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db_type::DbType;
    use crate::table_row::TableRow;

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
    fn set_up_table_with_duplicates() -> MockTable {
        MockTable::new(
            vec!["id".to_string(), "name".to_string()],
            vec![
                TableRow::new(vec![DbType::Int(1), DbType::Text("Alice".to_string())]),
                TableRow::new(vec![DbType::Int(2), DbType::Text("Bob".to_string())]),
                TableRow::new(vec![DbType::Int(1), DbType::Text("Alice".to_string())]),
                TableRow::new(vec![DbType::Int(2), DbType::Text("Bob".to_string())]),
            ],
        )
    }

    fn set_up_table_no_duplicates() -> MockTable {
        MockTable::new(
            vec!["id".to_string(), "name".to_string()],
            vec![
                TableRow::new(vec![DbType::Int(1), DbType::Text("Alice".to_string())]),
                TableRow::new(vec![DbType::Int(2), DbType::Text("Bob".to_string())]),
            ],
        )
    }

    #[test]
    fn test_distinct_with_duplicates() {
        let table = set_up_table_with_duplicates();
        let columns = vec!["id".to_string(), "name".to_string()];
        let result = distinct(&table, &columns);
        assert_eq!(result.get_data().len(), 2); // Expecting 2 unique rows
    }

    #[test]
    fn test_distinct_no_duplicates() {
        let table = set_up_table_no_duplicates();
        let columns = vec!["id".to_string(), "name".to_string()];
        let result = distinct(&table, &columns);
        assert_eq!(result.get_data().len(), 2); // Expecting 2 rows, as there are no duplicates
    }
}
