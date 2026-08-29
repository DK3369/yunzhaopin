use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// PHPYun `phpyun_bank` — admin bank-transfer accounts (`set_payset::bank`).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: u64,
    pub name: String,
    pub bank_name: String,
    pub bank_number: String,
    pub bank_address: String,
}
