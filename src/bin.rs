use paystack_rs::{Paystack, Transaction};

fn main() {
    testing_stuff()
}

fn testing_stuff() {
    let paystack = paystack_rs::Paystack {
        bearer_key: "bla bla".to_string(),
        Transaction: Transaction {
            email: "oayomide@enyata.com".to_string(),
            amount: 1000,
            ..Default::default()
        },
        ..Default::default()
    };

    paystack
        .Transaction
        .initialize_transaction(paystack.bearer_key);
}
