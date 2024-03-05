use crate::condition::{Condition, Operator};
use crate::db_type::DbType;
use crate::table_row::TableRow;
use crate::ManipulateTable;

#[cfg(test)]
mod test {
    use super::*;
    use crate::rational_algebra::projection::projection;
    use crate::rational_algebra::selection::selection;
    struct MockTable {
        columns_name: Vec<String>,
        data: Vec<TableRow>,
    }

    impl ManipulateTable for MockTable {
        fn get_column_names(&self) -> &Vec<String> {
            &self.columns_name
        }

        fn get_data(&self) -> &Vec<TableRow> {
            &self.data
        }
    }
    fn set_up_table() -> MockTable {
        MockTable {
            columns_name: vec![
                "id".to_string(),
                "name".to_string(),
                "family_name".to_string(),
            ],
            data: vec![
                TableRow::new(vec![
                    DbType::Int(1),
                    DbType::Text("Alice".to_string()),
                    DbType::Text("Baum".to_string()),
                ]),
                TableRow::new(vec![
                    DbType::Int(2),
                    DbType::Text("Uncle".to_string()),
                    DbType::Text("Bob".to_string()),
                ]),
            ],
        }
    }
    #[test]
    fn test_simple_selection_equal() {
        let table = set_up_table();
        let condition = Condition::Simple {
            field: "name".to_string(),
            operator: Operator::Equals,
            value: DbType::Text("Alice".to_string()),
        };
        let result = selection(&table, &condition);
        assert_eq!(result.get_data().len(), 1);
        assert_eq!(
            result.get_data()[0].get_values(),
            &vec![
                DbType::Int(1),
                DbType::Text("Alice".to_string()),
                DbType::Text("Baum".to_string())
            ]
        );
    }
    #[test]
    fn test_simple_projection() {
        let table = set_up_table();
        let columns_to_save = vec!["name".to_string(), "family_name".to_string()];
        let result = projection(&table, &columns_to_save);
        assert_eq!(result.get_column_names().len(), 2);
        assert_eq!(
            result.get_data()[0].get_values(),
            &vec![
                DbType::Text("Alice".to_string()),
                DbType::Text("Baum".to_string())
            ]
        );
    }
    #[test]
    fn test_selection_greater_than() {
        let table = set_up_table();
        let condition = Condition::Simple {
            field: "id".to_string(),
            operator: Operator::GreaterThan,
            value: DbType::Int(1),
        };
        let result = selection(&table, &condition);
        assert_eq!(result.get_data().len(), 1);
        assert_eq!(
            result.get_data()[0].get_values(),
            &vec![
                DbType::Int(2),
                DbType::Text("Bob".to_string()),
                DbType::Text("Uncle".to_string())
            ]
        );
    }

    #[test]
    fn test_selection_and_condition() {
        let table = set_up_table();
        let condition = Condition::And(
            Box::new(Condition::Simple {
                field: "id".to_string(),
                operator: Operator::GreaterThan,
                value: DbType::Int(0),
            }),
            Box::new(Condition::Simple {
                field: "name".to_string(),
                operator: Operator::Equals,
                value: DbType::Text("Alice".to_string()),
            }),
        );
        let result = selection(&table, &condition);
        assert_eq!(result.get_data().len(), 1);
        assert_eq!(
            result.get_data()[0].get_values(),
            &vec![
                DbType::Int(1),
                DbType::Text("Alice".to_string()),
                DbType::Text("Baum".to_string()),
            ]
        );
    }

    #[test]
    fn test_projection_nonexistent_column() {
        let table = set_up_table();
        let columns_to_project = vec!["name".to_string(), "nonexistent_column".to_string()];
        let result = projection(&table, &columns_to_project);
        assert_eq!(result.get_column_names().len(), 1);
        assert!(result.get_column_names().contains(&"name".to_string()));
        assert_eq!(
            result.get_data()[0].get_values(),
            &vec![DbType::Text("Alice".to_string()),]
        );
    }
}
