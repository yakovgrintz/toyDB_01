use crate::db_type::DbType;
use crate::metadata::MetaData;
use crate::table_row::TableRow;
use crate::ManipulateTable;

pub(crate) struct Table {
    meta_data: MetaData,
    data: Vec<TableRow>,
    column_names: Vec<String>,
    column_types: Vec<DbType>,
}
impl ManipulateTable for Table {
    fn get_column_names(&self) -> &Vec<String> {
        &self.column_names
    }

    fn get_data(&self) -> &Vec<TableRow> {
        &self.data
    }
}

impl Table {
    pub(crate) fn new(
        meta_data: MetaData,
        column_names: Vec<String>,
        column_types: Vec<DbType>,
    ) -> Table {
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
                (DbType::Int(_), DbType::Int(_)) => (),
                (DbType::Text(_), DbType::Text(_)) => (),
                (_, _) => panic!("type mismatch"),
            }
        }
        self.data.push(TableRow::new(data))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn set_up_table() -> Table {
        let meta_data = MetaData {}; // Assume a default or new instance can be made
        let mut column_names: Vec<String> = Vec::new();
        column_names.push("id".parse().unwrap());
        column_names.push("name".parse().unwrap());

        let mut column_types = vec![DbType::Int(0), DbType::Text(String::new())];
        Table::new(meta_data, column_names, column_types)
    }

    #[test]
    fn test_new_table() {
        let table = set_up_table();
        assert_eq!(table.column_names.len(), 2);
        assert_eq!(table.column_types.len(), 2);
    }
    #[test]
    fn test_insert_valid_data() {
        let mut table = set_up_table(); // Assume this is set up correctly
        table.insert(vec![DbType::Int(1), DbType::Text("Alice".to_string())]);
        assert_eq!(table.data.len(), 1);
    }

    #[test]
    #[should_panic(expected = "type mismatch")]
    fn test_insert_type_mismatch() {
        let mut table = set_up_table();
        table.insert(vec![DbType::Text("1".to_string()), DbType::Int(2)]); // Assuming a mismatch
    }
}
