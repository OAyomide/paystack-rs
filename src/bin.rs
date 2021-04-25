use paystack_rs::{Paystack, Transaction};
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
    let paystack = paystack_rs::Paystack::new("sdjskjdf".to_string());
    let body = paystack_rs::InitializeTransactionBody {
        amount: 10,
        email: "oayomide@enyata.com".to_string(),
        ..Default::default()
    };
    let response: Result<InitializeTxResponse, Error> = paystack
        .transaction
        .initialize_transaction(body)
        .unwrap()
        .json();
    println!("Heere the intialize tx body: {:?}", response.unwrap());
}
