#[derive(Clone,PartialEq,PartialOrd)]
pub(crate) enum DbType {
    Int(i32),
    Text(String),
}
