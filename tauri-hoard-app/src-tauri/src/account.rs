use serde::{Deserialize, Serialize};
use std::hash::Hash;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum AccountKind {
    Check,
    Save,
    Invest,
    I529,
    Other,
    Debt,
}

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Account {
    pub name: String,
    pub kind: AccountKind,
    pub id: Option<Uuid>,
}
