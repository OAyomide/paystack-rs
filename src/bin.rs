use paystack_rs::prelude::{InitializeTransactionBody, Paystack};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;
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

#[derive(Debug, Deserialize)]
struct ListAllBodyResponse {
    status: bool,
    message: String,
    data: Vec<JSON>,
    meta: JSON,
}
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ListAllBody {
    per_page: String,
    page: String,
}

fn testing_stuff() {
    // replace with a valid test keys
    let paystack = Paystack::new("some_key_here".to_string());
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
    let list_all_body = {};

    let list_all: Result<ListAllBodyResponse, Error> = paystack
        .subaccounts
        .list_subaccounts(Some(list_all_body))
        .unwrap()
        .json();

    println!("Result is: {:?}", list_all);
}
