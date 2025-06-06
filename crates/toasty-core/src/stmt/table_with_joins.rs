use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TableWithJoins {
    /// Identify a table
    pub table: TableRef,

    /// Joins to apply
    pub joins: Vec<Join>,
}
