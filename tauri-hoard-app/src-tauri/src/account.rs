use serde::{Deserialize, Serialize};
// use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountKind {
    Check,
    Save,
    Invest,
    I529,
    Other,
    Debt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub kind: AccountKind,
    pub balance: f32,
}
