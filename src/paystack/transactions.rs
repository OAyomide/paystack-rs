use crate::utils::*;
use chrono::{prelude::DateTime, Utc};
use reqwest::{
    blocking::{Client, Response},
    header::AUTHORIZATION,
    StatusCode,
};
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;

#[derive(Debug, Serialize)]
pub enum Currency {
    NGN,
    GHS,
    USD,
    ZAR,
}

#[derive(Debug, Serialize)]
pub enum Status {
    FAILED,
    SUCCESS,
    ABANDONED,
}

#[derive(Debug, Serialize)]
pub enum ChargesBearer {
    Account,
    Subaccount,
}
impl Default for ChargesBearer {
    fn default() -> Self {
        ChargesBearer::Account
    }
}
impl Default for Currency {
    fn default() -> Self {
        Currency::NGN
    }
}

impl std::fmt::Display for InitializeTransactionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

const CHARGE_AUTHORIZATION_URL: &str = "https://api.paystack.co/transaction/charge_authorization";
const INITIALIZE_TRANSACTION_URL: &str = "https://api.paystack.co/transaction/initialize";
const PAYSTACK_BASE_URL: &str = "https://api.paystack.co";
const TRANSACTION_URL: &str = "https://api.paystack.co/transaction";
/// struct passed to initiatialize a transaction.
#[derive(Debug, Default, Serialize)]
pub struct InitializeTransactionBody {
    pub email: String,
    pub amount: i128, // tbh, not sure what integer type i should use here. but pretty sure you cannot go wrong with i128
    pub currency: Option<Currency>,
    pub reference: Option<String>,
    pub callback_url: Option<String>,
    pub plan: Option<String>,
    pub invoice_limit: Option<i64>,
    pub metadata: Option<String>,
    pub channels: Option<Vec<String>>,
    pub split_code: Option<String>,
    pub subaccount: Option<String>,
    pub transaction_charge: Option<i128>,
    pub bearer: Option<ChargesBearer>,
}

/// struct ListTransactionsQuery
#[derive(Serialize)]
pub struct ListTransactionsQueryBody {
    pub per_page: Option<i64>,
    pub page: Option<i64>,
    pub customer: Option<i64>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub amount: Option<i128>,
}

#[derive(Serialize, Debug)]
pub struct ChargeAuthorizationBody {
    pub amount: String,
    pub email: String,
    pub authorization_code: String,
    pub reference: Option<String>,
    pub currency: Option<Currency>,
    pub metadata: Option<Value>,
    /// from the docs:
    /// Send us 'card' or 'bank' or 'card','bank' as an array to specify what options to show the user paying
    pub channels: Option<Vec<String>>,
    pub subaccount: Option<String>,
    pub transaction_charge: Option<i128>,
    pub bearer: Option<ChargesBearer>,
    /// If you are making a scheduled charge call, it is a good idea to queue them so the processing system does not get overloaded causing transaction processing errors. Send queue:true to take advantage of our queued charging.
    pub queue: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TransactionsTotal {
    pub per_page: Option<i64>,
    pub page: Option<i64>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CheckAuthorizationBody {
    pub amount: String,
    pub email: String,
    pub authorization_code: String,
    pub currency: Option<Currency>,
}

#[derive(Debug, Serialize)]
pub struct PartialDebitBody {
    pub amount: String,
    pub email: String,
    pub authorization_code: String,
    pub currency: Option<Currency>,
    pub reference: Option<String>,
    pub at_least: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExportTransactionsBody {
    pub per_page: Option<i64>,
    pub page: Option<i64>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub customer: Option<i64>,
    pub status: Option<Status>,
    pub currency: Option<Currency>,
    pub amount: Option<String>,
    pub settled: Option<bool>,
    pub settlement: Option<i64>,
    pub payment_page: Option<i64>,
}

#[derive(Default)]
pub struct Transaction {
    pub(crate) bearer_auth: String,
}

// WHY DO I KEEP USING .to_string() everywhere when i could just use &str?
// well because I honestly wish to appease rustc as best as i can. this means i do not want to start
// dealing with lifetime issues. Personal rule: avoid &str as best as you can.
impl Transaction {
    /// initialize a transaction.
    pub fn initialize_transaction(
        &self,
        body: InitializeTransactionBody,
    ) -> Result<Response, String> {
        let res = make_request(
            self.bearer_auth.clone(),
            INITIALIZE_TRANSACTION_URL.to_owned(),
            Some(body),
            REQUEST::POST,
        );
        return res;
    }

    /// verify a transaction. it takes an argument reference which is the reference_id of a transaction you want to verify
    pub fn verify_transaction(&self, reference: String) -> Result<Response, String> {
        let full_url = format!(
            "{}/transaction/verify/:{}",
            PAYSTACK_BASE_URL.to_owned(),
            reference.to_string()
        );
        let result = make_get_request(self.bearer_auth.clone(), full_url, None::<String>);
        return result;
    }

    /// list_transactions lists all the transactions available
    pub fn list_transactions(&self, body: ListTransactionsQueryBody) -> Result<Response, String> {
        let reqwest_client = Client::new();
        let res = reqwest_client
            .get(TRANSACTION_URL.to_owned())
            .header(AUTHORIZATION, self.bearer_auth.clone())
            .query(&[
                ("per_page".to_string(), body.per_page),
                ("page".to_string(), body.page),
                ("customer".to_string(), body.customer),
            ])
            .query(&[("from".to_string(), body.from, "to".to_string(), body.to)])
            .send()
            .expect("Error listing transactions. Please make sure you're doing the right thing");
        match res.status() {
            StatusCode::OK => return Ok(res),
            StatusCode::BAD_REQUEST => return Err("Bad request. Please check the body".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR => {
                return Err("An error occured on the paystack server: please try again".to_string())
            }
            _ => return Ok(res),
        }
    }

    pub fn fetch_transaction(&self, transaction_id: i64) -> Result<Response, String> {
        let reqwest_client = Client::new();
        let url = format!("{}/{}", TRANSACTION_URL.to_owned(), transaction_id);
        let res = reqwest_client
            .get(url)
            .header(AUTHORIZATION, self.bearer_auth.clone())
            .send()
            .expect("Error fetching all transactions");

        match res.status() {
            StatusCode::UNAUTHORIZED => {
                println!(
                    "Oops! Unauthorized request. Please ensure you've set the correct headers"
                );
                return Err("Unauthorized request. please check header values".to_string());
            }
            StatusCode::BAD_REQUEST => return Err(
                "Bad request. Please check whatever you're passing in the request. Seems broken"
                    .to_string(),
            ),
            StatusCode::OK => {
                println!("Yay!! you got it!!");
                return Ok(res);
            }
            _ => {
                // the below is meant as a light joke.. chill out pls
                println!("Dunno... Looks Ok but since its not an error i specially check for, here is your result, man... or woman... or they/them");
                return Ok(res);
            }
        };
    }

    pub fn charge_authorization(
        &self,
        params: ChargeAuthorizationBody,
    ) -> Result<Response, String> {
        let res = make_request(
            self.bearer_auth.clone(),
            CHARGE_AUTHORIZATION_URL.to_owned(),
            Some(params),
            REQUEST::POST,
        );
        return res;
    }
    /// ⚠️ Warning You shouldn't use this endpoint to check a card for sufficient funds if you are going to charge the user immediately. This is because we hold funds when this endpoint is called which can lead to an insufficient funds error.
    pub fn check_authorization(&self, param: ChargeAuthorizationBody) -> Result<Response, String> {
        let full_url = CHARGE_AUTHORIZATION_URL.to_owned();
        // let res = make_post_request(self.bearer_auth.clone(), full_url, param);
        let res = make_request(
            self.bearer_auth.clone(),
            full_url,
            Some(param),
            REQUEST::POST,
        );
        return res;
    }

    pub fn view_transaction_timeline(&self, id: String) -> Result<Response, String> {
        let full_url = format!("{}/timeline/{}", TRANSACTION_URL.to_owned(), id).to_string();
        let res = make_get_request(self.bearer_auth.clone(), full_url, None::<String>);
        return res;
    }

    pub fn transactions_total(
        &self,
        params: Option<TransactionsTotal>,
    ) -> Result<Response, String> {
        let full_url = format!("{}/totals", TRANSACTION_URL.to_owned());
        let reqwest_client = Client::new();
        let params = params.expect("Error unwrapping params");
        let res = reqwest_client
            .get(full_url)
            .query(&[
                ("perPage".to_string(), params.per_page.unwrap()),
                ("page".to_string(), params.page.unwrap()),
            ])
            .header(AUTHORIZATION, self.bearer_auth.clone())
            .send()
            .expect("Error fetching transactions total");

        match res.status() {
            StatusCode::OK => return Ok(res),
            StatusCode::BAD_REQUEST => return Err("Bad request. Please check the body".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR => {
                return Err("An error occured on the paystack server: please try again".to_string())
            }
            _ => return Ok(res),
        }
    }

    pub fn export_transactions(
        &self,
        params: Option<ExportTransactionsBody>,
    ) -> Result<Response, String> {
        let full_url = format!("{}/export", TRANSACTION_URL.to_owned());
        let params = params.expect("Error unwrapping params passed to export transactions");
        let reqwest_client = Client::new();
        let res = reqwest_client
            .get(full_url)
            .query(&[
                ("perPage".to_string(), params.per_page.unwrap()),
                ("page".to_string(), params.page.unwrap()),
                ("customer".to_string(), params.customer.unwrap()),
                ("payment_page".to_string(), params.payment_page.unwrap()),
                ("settlement".to_string(), params.settlement.unwrap()),
            ])
            .query(&[
                ("from".to_string(), params.from.unwrap()),
                ("to".to_string(), params.to.unwrap()),
            ])
            .query(&[("status".to_string(), params.status.unwrap())])
            .query(&[("amount".to_string(), params.amount.unwrap())])
            .query(&[("currency".to_string(), params.currency.unwrap())])
            .query(&[("settled".to_string(), params.settled.unwrap())])
            .header(AUTHORIZATION, self.bearer_auth.clone())
            .send()
            .expect("Error fetching transactions total");

        match res.status() {
            StatusCode::OK => return Ok(res),
            StatusCode::BAD_REQUEST => return Err("Bad request. Please check the body".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR => {
                return Err("An error occured on the paystack server: please try again".to_string())
            }
            _ => return Ok(res),
        }
    }

    pub fn partial_debit(&self, body: PartialDebitBody) -> Result<Response, String> {
        let full_url = format!("{}/partial_debit", TRANSACTION_URL.to_owned());
        let res = make_request(
            self.bearer_auth.clone(),
            full_url,
            Some(body),
            REQUEST::POST,
        );
        return res;
    }
}
