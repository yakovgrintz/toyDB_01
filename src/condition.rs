use crate::db_type::DbType;

pub(crate) enum Condition {
    Simple {
        field: String,
        operator: Operator,
        value: DbType,
    },
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}
pub(crate) enum Operator {
    Equals,
    LessThan,
    GreaterThan,
    NotEquals,
}
