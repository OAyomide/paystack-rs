pub mod transactions;

use transactions::Transaction;
#[derive(Default)]
pub struct Paystack {
    pub transaction: Transaction,
}

impl Paystack {
    pub fn new(key: String) -> Paystack {
        let formatted_bearer = format!("Bearer {}", key);
        Paystack {
            transaction: Transaction {
                bearer_auth: formatted_bearer.to_string(),
                ..Default::default()
            },
        }
    }
}
