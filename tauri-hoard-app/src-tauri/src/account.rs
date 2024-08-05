use serde::{Deserialize, Serialize};
use std::{default, hash::Hash};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum AccountKind {
    Check,
    Save,
    Invest,
    I529,
    Debt,
    #[default]
    Other,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Account {
    pub name: String,
    pub kind: AccountKind,
    pub id: Option<Uuid>,
}
