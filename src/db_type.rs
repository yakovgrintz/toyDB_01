use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize, Hash, Eq)]
pub(crate) enum DbType {
    Int(i32),
    Text(String),
}
