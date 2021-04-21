# Paystack-rs

A rust crate that wraps around the paystack REST API. [WIP]. Not ready for use yet.

## Whats working and whats yet to be done?

Currently, there are methods for every transactions endpoint. Anything apart from that is not ready yet. I recently started working on it and since I have a full-time job that takes my time, I basically just try to write like a few minutes or an hour a day of open-source code and this is one. Therefore, I still have a lot of things to do.

Some things to be done (Endpoints) are:

- [x] Transactions
- [ ] Transactions splits
- [ ] Customers
- [ ] Dedicated NUBAN
- [ ] Subaccounts
- [ ] Plans
- [ ] Subscription
- [ ] Products
- [ ] Payment pages
- [ ] Invoices
- [ ] Settlement
- [ ] Transfer recipients
- [ ] Transfers
- [ ] Transfers Control
- [ ] Bulk Charges
- [ ] Control Panel
- [ ] Charge
- [ ] Dispute
- [ ] Refunds
- [ ] Verifications
- [ ] Miscellaneous

Other things are:

- [ ] Tests (i know!! tests are important! 🙃)
- [ ] Support reading Bearer access token and similar sensitive information from .env file
- [ ] Proper comments and documentation

## How to use

```rust
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
```

The above is taken from the `bin.rs` file. It imports the crate, takes the access token and returns an "instance" of paystack. It then creates the transactions body that the `initialize_transaction()` method (for Paystack's Initialize Transaction endpoint) takes. it then then calls the method as needed. Quite obvious and straightforward.
