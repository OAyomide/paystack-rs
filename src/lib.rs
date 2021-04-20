use chrono::{prelude::DateTime, Utc};
use reqwest::{
    blocking::{Client, Response},
    header::{AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use serde::Serialize;
use std::fmt::Debug;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug, Serialize)]
pub enum Currency {
    NGN,
    GHS,
    USD,
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

#[derive(Default)]
pub struct Paystack {
    // pub bearer_key: String,
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

#[derive(Default)]
pub struct Transaction {
    bearer_auth: String,
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
        let base_url = "https://api.paystack.co/transaction/initialize".to_string();

        let res = self.make_post_request(base_url, body);
        return res;
    }

    /// verify a transaction. it takes an argument reference which is the reference_id of a transaction you want to verify
    pub fn verify_transaction(&self, reference: String) -> Result<Response, String> {
        let base_url = "https://api.paystack.co".to_string();
        let full_url = format!("{}/transaction/verify/:{}", base_url, reference.to_string());
        let result = self.make_get_request(full_url);
        return result;
    }

    /// list_transactions lists all the transactions available
    pub fn list_transactions(&self, body: ListTransactionsQueryBody) -> Result<Response, String> {
        let full_url = "https://api.paystack.co/transaction";
        let reqwest_client = Client::new();
        let res = reqwest_client
            .get(full_url)
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
        let url = format!("https://api.paystack.co/transaction/{}", transaction_id);
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

    fn make_get_request(&self, url: String) -> Result<Response, String> {
        let reqwest_client = Client::new();
        let formatted_err_msg = format!(
            "[PAYSTACK ERROR]: Error making GET request to url: {}",
            url.to_string()
        );
        let res = reqwest_client
            .get(url)
            .header(AUTHORIZATION, self.bearer_auth.clone())
            .send()
            .expect(formatted_err_msg.as_str());

        match res.status() {
            StatusCode::OK => return Ok(res),
            StatusCode::BAD_REQUEST => return Err("Bad request. Please check the body".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR => {
                return Err("An error occured on the paystack server: please try again".to_string())
            }
            _ => return Ok(res),
        }
    }

    fn make_post_request<T>(&self, url: String, body: T) -> Result<Response, String>
    where
        T: Debug + Serialize,
    {
        let reqwest_client = Client::new();
        let formatted_err_msg = format!(
            "[PAYSTACK ERROR]: Error making POST request to paystack with URL: {} and body: {:?}",
            url, body
        );

        let serialized_body =
            serde_json::to_string(&body).expect("Error serializing POST request body");
        let res = reqwest_client
            .post(url)
            .header(AUTHORIZATION, self.bearer_auth.clone())
            .header(CONTENT_TYPE, "application/json".to_string())
            .body(serialized_body)
            .send()
            .expect(formatted_err_msg.as_str());

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
}
