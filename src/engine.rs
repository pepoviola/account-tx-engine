use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

use crate::account::Account;
use crate::transaction::{Transaction, TransactionStatus, TransactionType};
use crate::transaction_row::TransactionRow;
use crate::amount::Amount;

#[derive(Debug)]
pub struct PaymentEngine {
    tx_by_txid: HashMap<u32, Transaction>,
    accounts_by_client: HashMap<u16, Account>,
}
impl PaymentEngine {
    pub fn new() -> PaymentEngine {
        PaymentEngine {
            tx_by_txid: Default::default(),
            accounts_by_client: Default::default(),
        }
    }

    pub fn process_from_reader(
        &mut self,
        mut reader: csv::Reader<File>,
    ) -> Result<(), Box<dyn Error>> {
        while let Some(result) = reader.deserialize::<TransactionRow>().next() {
            let tx_record = result?;
            if let Some(tx) = tx_record.into_transaction() {
                self.apply_transaction(tx);
            }
        }
        Ok(())
    }

    pub fn apply_transaction(&mut self, mut tx: Transaction) {
        match tx.tx_type {
            TransactionType::Deposit => {
                if self.apply_deposit(&tx) {
                    tx.status = TransactionStatus::Processed
                } else {
                    tx.status = TransactionStatus::Failed
                }
                self.tx_by_txid.insert(tx.id, tx);
            }
            TransactionType::Withdrawal => {
                if self.apply_withdrawal(&tx) {
                    tx.status = TransactionStatus::Processed
                } else {
                    tx.status = TransactionStatus::Failed
                }
                self.tx_by_txid.insert(tx.id, tx);
            }
            TransactionType::Dispute => {
                self.apply_dispute(&tx);
            }
            TransactionType::Resolve => self.apply_resolve(&tx),
            TransactionType::Chargeback => self.apply_chargeback(&tx),
        };
    }

    pub fn output_report(&self) {
        println!("client, available, held, total, locked");

        for (_, account) in self.accounts_by_client.iter() {
            let total = Amount {
                inner: account.available + account.held,
            };

            println!(
                "{:?}, {:?}, {:?}, {:?}, {:?}",
                account.client,
                Amount::new(account.available).to_output(),
                Amount::new(account.held).to_output(),
                total.to_output(),
                account.locked
            );
        }
    }

    fn apply_deposit(&mut self, tx: &Transaction) -> bool {
        let account = self
            .accounts_by_client
            .entry(tx.account)
            .or_insert_with(|| Account::new_with_client(tx.account));

        if account.locked {
            eprintln!("Account locked");
            return false;
        }

        if let Some(amount) = &tx.amount {
            account.available += amount.value();
            true
        } else {
            false
        }
    }

    fn apply_withdrawal(&mut self, tx: &Transaction) -> bool {
        let account = self
            .accounts_by_client
            .entry(tx.account)
            .or_insert_with(|| Account::new_with_client(tx.account));

        if account.locked {
            eprintln!("Account locked");
            return false;
        }

        if let Some(amount) = &tx.amount {
            if account.available >= amount.value() {
                account.available -= amount.value();
            }

            true
        } else {
            false
        }
    }

    fn apply_dispute(&mut self, tx: &Transaction) {
        let account = self
            .accounts_by_client
            .entry(tx.account)
            .or_insert_with(|| Account::new_with_client(tx.account));

        if account.locked {
            eprintln!("Account locked");
            return;
        }

        if let Some(original_tx) = self.tx_by_txid.get_mut(&tx.id) {
            if original_tx.status != TransactionStatus::Processed {
                eprintln!(
                    "Ignoring dispute in transaction {} for client {}, wasn't processed",
                    original_tx.id, original_tx.account
                );
                return;
            }

            match original_tx.tx_type {
                TransactionType::Deposit => {
                    if let Some(amount) = original_tx.amount.as_ref() {
                        account.available -= amount.value();
                        account.held += amount.value();

                        original_tx.status = TransactionStatus::Disputed;
                    }
                }
                TransactionType::Withdrawal => {
                    if let Some(amount) = original_tx.amount.as_ref() {
                        account.available -= amount.value();
                        account.held += amount.value();

                        original_tx.status = TransactionStatus::Disputed;
                    }
                }
                _ => {
                    eprintln!("Ignoring dispute in transaction {} for client {}, was not related to a deposit or withdrawal", original_tx.id, original_tx.account);
                    return;
                }
            }
        }
    }

    fn apply_resolve(&mut self, tx: &Transaction) {
        let account = self
            .accounts_by_client
            .entry(tx.account)
            .or_insert_with(|| Account::new_with_client(tx.account));

        if account.locked {
            eprintln!("Account locked");
            return;
        }

        if let Some(original_tx) = self.tx_by_txid.get_mut(&tx.id) {
            if original_tx.status != TransactionStatus::Disputed {
                eprintln!(
                    "Ignoring dispute in transaction {} for client {}, wasn't disputed",
                    original_tx.id, original_tx.account
                );
                return;
            }

            if let Some(amount) = original_tx.amount.as_ref() {
                account.available += amount.value();
                account.held -= amount.value();

                original_tx.status = TransactionStatus::Processed;
            }
        }
    }

    fn apply_chargeback(&mut self, tx: &Transaction) {
        let account = self
            .accounts_by_client
            .entry(tx.account)
            .or_insert_with(|| Account::new_with_client(tx.account));

        if let Some(original_tx) = self.tx_by_txid.get_mut(&tx.id) {
            if original_tx.status != TransactionStatus::Disputed {
                eprintln!(
                    "Ignoring chargeback in transaction {} for client {}, wasn't disputed",
                    original_tx.id, original_tx.account
                );
                return;
            }
            if let Some(amount) = original_tx.amount.as_ref() {
                account.available += amount.value();
                account.held -= amount.value();

                original_tx.status = TransactionStatus::ChargedBacked;
            }

            account.locked = true;
        }
    }
}
