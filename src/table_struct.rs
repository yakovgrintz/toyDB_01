use crate::db_type::DbType;
use crate::metadata::MetaData;
use crate::query_result::QueryResult;
use crate::row_struct::TableRow;
use crate::ManipulateTable;

struct Table {
    meta_data: MetaData,
    data: Vec<TableRow>,
    column_names: Vec<String>,
    column_types: Vec<DbType>,
}
impl ManipulateTable for Table {
    fn get_column_names(&self)->Vec<String> {
        self.column_names.clone()
    }

    fn get_data(&self)->Vec<TableRow> {
        self.data.clone()
    }
}

impl Table {
    fn new(meta_data: MetaData, column_names: Vec<str>, column_types: Vec<DbType>) -> Table {
        assert_eq!(column_names.len(), column_types.len());
        Table {
            meta_data,
            data: vec![],
            column_names,
            column_types,
        }
    }
    fn insert(&mut self, data: Vec<DbType>) {
        assert_eq!(
            data.len(),
            self.column_types.len(),
            "Data length does not match column length."
        );

        for (data, column_type) in data.iter().zip(self.column_types.iter()) {
            match (data, column_type) {
                //add data type to check with this pattern: (DbType::Int(_), DbType::Int(_)) => (),
                (_, _) => panic!("type mismatch"),
            }
        }
        self.data.push(TableRow::new(data));
    }

    fn selection(self, condition: &str) -> QueryResult {
        let tokens = condition.split_whitespace().collect();
        let field: &str = tokens[0];
        let operator = tokens[1];
        let value = tokens[2];
        let index = self.column_names.iter().position(|c| c == field);
        let result: Vec<TableRow> = self
            .data
            .iter()
            .map(|row| {
                let row_value = row.get_values().get(index).expect("Index out of bounds");
                let matches = match operator {
                    "<" => row.get_values().get(index) < value,
                    ">" => row.get_values().get(index) > value,
                    "=" => row.get_values().get(index) == value,
                    "<=>" => row.get_values().get(index) <= value,
                    ">=" => row.get_values().get(index) >= value,
                    "!=" => row.get_values().get(index) != value,
                };
                if matches {
                    Some(row.clone())
                } else {
                    None
                }
            })
            .collect();
        QueryResult { data: vec![], column_names: vec![] }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_table() {
        let meta_data = MetaData {}; // Assume a default or new instance can be made
        let column_names = vec!["id", "name"];
        let column_types = vec![DbType::Int, DbType::Text]; // Simplified assumption

        let table = Table::new(meta_data, column_names, column_types);
        assert_eq!(table.column_names.len(), 2);
        assert_eq!(table.column_types.len(), 2);
    }
    #[test]
    fn test_insert_valid_data() {
        let mut table = Table::new(/* parameters */); // Assume this is set up correctly
        table.insert(vec![DbType::Int(1), DbType::Text("Alice".to_string())]);
        assert_eq!(table.data.len(), 1);
    }

    #[test]
    #[should_panic(expected = "type mismatch")]
    fn test_insert_type_mismatch() {
        let mut table = Table::new(/* parameters */);
        table.insert(vec![DbType::Text("1".to_string()), DbType::Int(2)]); // Assuming a mismatch
    }
    #[test]
    fn test_projection_valid_columns() {
        let table = Table::setup_with_data(); // This would be a helper function to setup a table with data
        let projected = table.projection(&["name".to_string()]);
        assert_eq!(projected.data.len(), table.data.len()); // Simplify: Check if result matches expected
    }
    #[test]
    fn test_selection_with_condition() {
        let table = Table::setup_with_data(); // Setup with specific rows
        let selected = table.selection("id > 1");
        // Verify that selected contains rows matching the condition
    }
}
