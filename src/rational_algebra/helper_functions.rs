use crate::ManipulateTable;

pub(crate) fn find_indexes<T>(table: &T, columns: &[String]) -> Vec<usize>
where
    T: ManipulateTable,
{
    let column_names: &Vec<String> = table.get_column_names();
    let indexes: Vec<usize> = columns
        .iter()
        .filter_map(|name| column_names.iter().position(|c| c == name))
        .collect();
    indexes
}
