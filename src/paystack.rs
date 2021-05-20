pub mod customers;
pub mod dedicated_nuban;
pub mod subaccounts;
pub mod transactions;
pub mod transactions_split;

use dedicated_nuban::DedicatedNuban;
use subaccounts::Subaccount;
use transactions::Transaction;
use transactions_split::TransactionSplit;

#[derive(Default)]
pub struct Paystack {
    pub transaction: Transaction,
    pub transaction_split: TransactionSplit,
    pub subaccounts: Subaccount,
    pub dedicated_nuban: DedicatedNuban,
}

impl Paystack {
    pub fn new(key: String) -> Paystack {
        let formatted_bearer = format!("Bearer {}", key);
        Paystack {
            transaction: Transaction {
                bearer_auth: formatted_bearer.to_string(),
                ..Default::default()
            },
            transaction_split: TransactionSplit {
                bearer_auth: formatted_bearer.to_string(),
            },
            subaccounts: Subaccount {
                bearer_auth: formatted_bearer.to_string(),
            },
            dedicated_nuban: DedicatedNuban {
                bearer_auth: formatted_bearer.to_string(),
            },
        }
    }
}
