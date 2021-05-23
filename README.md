# Paystack-rs

A rust crate that wraps around the paystack REST API.

## Whats working and whats yet to be done?

APIs that have been implemented are:

- [x] Transactions
- [x] Transactions splits
- [x] Customers
- [x] Dedicated NUBAN
- [x] Subaccounts
- [x] Plans
- [x] Subscription
- [x] Products
- [x] Payment pages
- [x] Invoices
- [x] Settlement
- [x] Transfer recipients
- [x] Transfers
- [x] Transfers Control
- [x] Bulk Charges
- [x] Control Panel
- [x] Charge
- [x] Dispute
- [x] Refunds
- [x] Verifications
- [x] Miscellaneous

Other things are:

- [ ] Tests (i know!! tests are important! 🙃)
- [ ] Support reading Bearer access token and similar sensitive information from .env file

## How to use

```rust
use paystack_rs::prelude::{InitializeTransactionBody, Paystack};
use reqwest::Error;
use serde::Deserialize;
use std::result::Result;

fn main() {
    testing_stuff()
}

#[derive(Deserialize, Debug)]
struct InitializeTxResponse {
    status: bool,
    message: String,
    data: Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    authorization_url: String,
    access_code: String,
}
fn testing_stuff() {
    // replace with a valid test keys
    let paystack = Paystack::new("mmdmfkdfm".to_string());
    let body = InitializeTransactionBody {
        amount: 10,
        email: "oayomide@enyata.com".to_string(),
        ..Default::default()
    };
    let response: Result<InitializeTxResponse, Error> = paystack
        .transaction
        .initialize_transaction(body)
        .unwrap()
        .json();
    println!("Result: {:?}", response.unwrap());
    // InitializeTxResponse { status: true, message: "Authorization URL created", data: Data { authorization_url: "https://checkout.paystack.com/gx9mi6ihvnw5s9s", access_code: "gx9mi6ihvnw5s9s" } }
}
```

The above is taken from the `bin.rs` file. It imports the crate, takes the access token and returns an "instance" of paystack. It then creates the transactions body that the `initialize_transaction()` method (for Paystack's Initialize Transaction endpoint) takes. it then then calls the method as needed. Quite obvious and straightforward.
