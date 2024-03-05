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
    let index_key: HashMap<String, usize> = column_names
        .iter()
        .enumerate()
        .map(|(index, name)| (name.clone(), index))
        .collect();
    let evaluator = make_condition_evaluator(condition, &index_key);

    let result = table
        .get_data()
        .iter()
        .filter(|row| evaluator(row))
        .cloned()
        .collect();
    QueryResult::new(result, table.get_column_names().clone())
}

fn make_condition_evaluator<'a>(
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
                Some(row_value) => evaluate(row_value, operator, value),
                None => false,
            };
            Box::new(elevator)
        }
        Condition::And(lhs, rhs) => {
            let left_operand = make_condition_evaluator(lhs, index_key);
            let right_operand = make_condition_evaluator(rhs, index_key);
            Box::new(move |row| left_operand(row) && right_operand(row))
        }
        Condition::Or(lhs, rhs) => {
            let left_operand = make_condition_evaluator(lhs, index_key);
            let right_operand = make_condition_evaluator(rhs, index_key);
            Box::new(move |row| left_operand(row) || right_operand(row))
        }
    }
}

#[inline]
fn evaluate(row_value: &DbType, operator: &Operator, value: &DbType) -> bool {
    match operator {
        Operator::Equals => row_value == value,
        Operator::LessThan => row_value < value,
        Operator::GreaterThan => row_value > value,
        Operator::NotEquals => row_value != value,
    }
}
