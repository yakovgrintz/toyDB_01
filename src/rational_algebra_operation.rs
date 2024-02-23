use crate::condition::{Condition, Operator};
use crate::db_type::DbType;
use crate::query_result::QueryResult;
use crate::table_row::TableRow;
use crate::ManipulateTable;
use std::collections::HashMap;

pub(crate) fn selection<T>(table: &T, condition: &Condition) -> QueryResult
where
    T: ManipulateTable,
{
    let column_names = table.get_column_names();
    let mut index_key: HashMap<String, usize> = column_names
        .iter()
        .enumerate()
        .map(|(index, name)| (name.clone(), index))
        .collect();
    let evaluator = make__condition_evaluator(condition, &index_key);

    let result = table
        .get_data()
        .iter()
        .filter(|row| evaluator(row))
        .cloned()
        .collect();
    QueryResult::new(result, table.get_column_names().clone())
}

fn make__condition_evaluator<'a>(
    condition: &'a Condition,
    index_key: &'a HashMap<String, usize>,
) -> Box<dyn Fn(&TableRow) -> bool + 'a> {
    match condition {
        Condition::Simple {
            field,
            operator,
            value,
        } => {
            let field_index = *index_key.get(field).unwrap();
            let elevator = move |row: &TableRow| match row.get_values().get(field_index) {
                Some(row_value) => match (operator) {
                    Operator::Equals => row_value == value,
                    Operator::LessThan => row_value < value,
                    Operator::GreaterThan => row_value < value,
                    Operator::NotEquals => row_value != value,
                    _ => false,
                },
                None => false,
            };
            Box::new(elevator)
        }
        Condition::And(lhs, rhs) => {
            let left_operand = make__condition_evaluator(lhs, index_key);
            let right_operand = make__condition_evaluator(rhs, index_key);
            Box::new(move |row| left_operand(row) && right_operand(row))
        }
        Condition::Or(lhs, rhs) => {
            let left_operand = make__condition_evaluator(lhs, index_key);
            let right_operand = make__condition_evaluator(rhs, index_key);
            Box::new(move |row| left_operand(row) || right_operand(row))
        }
    }
}

pub(crate) fn projection<T>(table: T, column: Vec<String>) -> QueryResult
where
    T: ManipulateTable,
{
    assert!(table.get_column_names().len() >= column.len());
    let column_names: &Vec<String> = table.get_column_names();
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

#[cfg(test)]
mod test {
    use super::*;
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
        let result = projection(table, columns_to_save);
        assert_eq!(result.get_column_names().len(), 2);
        assert_eq!(
            result.get_data()[0].get_values(),
            &vec![
                DbType::Text("Alice".to_string()),
                DbType::Text("Baum".to_string())
            ]
        );
    }
}
