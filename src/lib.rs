use reqwest::{
    blocking::{Client, Response},
    header::{AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::from_value;

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

impl std::fmt::Display for TransactionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Default, Serialize)]
pub struct TransactionBody {
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

#[derive(Default)]
pub struct Paystack {
    // pub bearer_key: String,
    pub transaction: Transaction,
}

impl Paystack {
    pub fn new(key: String) -> Paystack {
        Paystack {
            transaction: Transaction {
                bearer_key: key,
                ..Default::default()
            },
        }
    }
}

#[derive(Default)]
pub struct Transaction {
    bearer_key: String,
}

// WHY DO I KEEP USING .to_string() everywhere when i could just use &str?
// well because I honestly wish to appease rustc as best as i can. this means i do not want to start
// dealing with lifetime issues. Personal rule: avoid &str as best as you can.
impl Transaction {
    pub fn initialize_transaction(&self, body: TransactionBody) -> Result<Response, String> {
        let base_url = "https://api.paystack.co".to_string();
        let endpoint = "/transaction/initialize".to_string();
        let reqwest_client = Client::new();
        let tx_body = serde_json::to_string(&body).expect("Error serializing body into JSON");
        let formatted_bearer = format!("Bearer {}", self.bearer_key.to_string());
        println!("Formatted header is: {:?}", formatted_bearer);
        let res = reqwest_client
            .post(format!("{}{}", base_url, endpoint))
            .header(AUTHORIZATION, formatted_bearer)
            .header(CONTENT_TYPE, "application/json".to_string())
            .body(tx_body)
            .send()
            .expect("Error initializing paystack request");

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

    pub fn verify_transaction(&self, reference: String) -> Result<Response, String> {
        let base_url = "https://api.paystack.co".to_string();
        let full_url = format!("{}/transaction/verify/:{}", base_url, reference.to_string());

        let client = Client::new();
        let token = format!("Bearer {}", self.bearer_key);
        let res = client
            .get(full_url)
            .header(AUTHORIZATION, token)
            .send()
            .expect("Error verifying transaction from paystack");

        match res.status() {
            StatusCode::OK => {
                println!("Ok, the transaction checks out");
                return Ok(res);
            }
            StatusCode::BAD_REQUEST => {
                return Err(
                    "Bad request. Please check that you're passing the correct parameters"
                        .to_string(),
                );
            }
            _ => return Ok(res);
        }
    }
}
