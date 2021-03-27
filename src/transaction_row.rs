use std::str::FromStr;

use crate::transaction::{Transaction, TransactionStatus, TransactionType};
use crate::amount::Amount;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionRow {
    #[serde(rename = "type")]
    pub tx_type: String,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
}

impl TransactionRow {
    pub fn into_transaction(&self) -> Option<Transaction> {
        let tx_type = match TransactionType::from_str(&self.tx_type) {
            Ok(tx_type) => tx_type,
            Err(_) => {
                eprintln!("Invalid transaction type {}, skkiping row", self.tx_type);
                return None;
            }
        };
        let amount = match self.amount {
            Some(a) => Some(Amount::from_input(a)),
            None => None,
        };

        let tx = Transaction {
            amount,
            tx_type,
            id: self.tx,
            account: self.client,
            status: TransactionStatus::Unprocessed,
        };

        Some(tx)
    }
}
