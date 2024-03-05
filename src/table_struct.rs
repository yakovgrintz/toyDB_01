use crate::db_type::DbType;
use crate::metadata::MetaData;
use crate::table_row::TableRow;
use crate::ManipulateTable;

pub(crate) struct Table {
    name_of_table: String,
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
        name_of_table: String,
        meta_data: MetaData,
        column_names: Vec<String>,
        column_types: Vec<DbType>,
    ) -> Self {
        assert_eq!(column_names.len(), column_types.len());
        let capacity = meta_data.get_table_capacity();
        Table {
            name_of_table,
            meta_data,
            data: Vec::with_capacity(capacity),
            column_names,
            column_types,
        }
    }
    pub(crate) fn set_primary_key(&mut self, column: &[String]) {
        let indices: Vec<usize> = column
            .iter()
            .filter_map(|name| self.column_names.iter().position(|c| c == name))
            .collect();
        self.meta_data.set_pk(&indices);
    }
    fn insert(&mut self, data: Vec<DbType>) {
        assert_eq!(
            data.len(),
            self.column_types.len(),
            "Data length does not match column length."
        );

        for (data, column_type) in data.iter().zip(self.column_types.iter()) {
            match (data, column_type) {
                (DbType::Int(_), DbType::Int(_)) | (DbType::Text(_), DbType::Text(_)) => (),
                (_, _) => panic!("type mismatch"),
            }
        }
        //check if primary key exist
        let mut result: Vec<DbType> = Vec::new();

        for &index in self.meta_data.get_pk() {
            if let Some(value) = data.get(index) {
                result.push(value.clone());
            }
        }

        let filter = self.meta_data.get_filter();
        if filter.check(&result) {
            panic!("Inserting a value that already exists (with high probability) into the primary key column. Please enter another value");
        } else {
            self.data.push(TableRow::new(data));
            filter.add(&result);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn set_up_table() -> Table {
        // Assume a default or new instance can be made
        let mut column_names: Vec<String> = Vec::new();
        column_names.push("id".parse().unwrap());
        column_names.push("name".parse().unwrap());

        let mut column_types = vec![DbType::Int(0), DbType::Text(String::new())];
        Table::new(
            "Test".to_string(),
            set_up_meta_data(),
            column_names,
            column_types,
        )
    }
    fn set_up_meta_data() -> MetaData {
        MetaData::new(20, 0.05)
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
    #[test]
    #[should_panic(expected = "Inserting a value that already exists")]
    fn test_insert_duplicate_primary_key() {
        let mut table = set_up_table();
        table.set_primary_key(&vec!["id".to_string()]);
        table.insert(vec![DbType::Int(1), DbType::Text("Alice".to_string())]);
        // This should panic because it attempts to insert a duplicate primary key.
        table.insert(vec![DbType::Int(1), DbType::Text("Bob".to_string())]);
    }
}
