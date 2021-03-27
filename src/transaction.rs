use crate::amount::Amount;
use std::str::FromStr;

#[derive(Debug)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl FromStr for TransactionType {
    type Err = String;

    fn from_str(tx_type_as_str: &str) -> Result<Self, Self::Err> {
        match tx_type_as_str {
            "deposit" => Ok(TransactionType::Deposit),
            "withdrawal" => Ok(TransactionType::Withdrawal),
            "dispute" => Ok(TransactionType::Dispute),
            "resolve" => Ok(TransactionType::Resolve),
            "chargeback" => Ok(TransactionType::Chargeback),
            _ => Err(format!(
                "'{}' is not a valid TransactionType",
                tx_type_as_str
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TransactionStatus {
    Unprocessed,
    Processed,
    Failed,
    Disputed,
    ChargedBacked,
}
#[derive(Debug)]
pub struct Transaction {
    pub id: u32,
    pub tx_type: TransactionType,
    pub account: u16,
    pub amount: Option<Amount>,
    pub status: TransactionStatus,
}
