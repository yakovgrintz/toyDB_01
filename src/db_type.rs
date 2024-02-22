#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub(crate) enum DbType {
    Int(i32),
    Text(String),
}
