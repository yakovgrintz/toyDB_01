use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize, Hash)]
pub(crate) enum DbType {
    Int(i32),
    Text(String),
}
