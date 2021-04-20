use paystack_rs::{Paystack, Transaction};

fn main() {
    testing_stuff()
}

fn testing_stuff() {
    let paystack = paystack_rs::Paystack::new("bla bla bla ".to_string());
    let body = paystack_rs::InitializeTransactionBody {
        amount: 10,
        email: "oayomide@enyata.com".to_string(),
        ..Default::default()
    };
    let ps = paystack.transaction.initialize_transaction(body);
}
