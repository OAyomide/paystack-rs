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
