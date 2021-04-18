use reqwest::{
    blocking::Client,
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
pub enum Currency {
    NGN,
    GHS,
    USD,
}

impl Default for Currency {
    fn default() -> Self {
        Currency::NGN
    }
}
#[derive(Default)]
pub struct Transaction {
    pub email: String,
    pub amount: i128,
    pub currency: Currency,
    pub reference: String,
    pub callback_url: String,
    pub plan: String,
    pub invoice_limit: i64,
}

#[derive(Debug, Default, Serialize)]
struct TransactionBody {
    email: String,
    amount: i128,
}

impl Transaction {
    pub fn list(&self) {
        print!("Heyy!!")
    }

    pub fn initialize_transaction(&self, token: String) {
        let default_url = "https://api.paystack.co".to_string();
        let endpoint = "/transaction/initialize".to_string();
        let reqwest_client = Client::new();
        let body = TransactionBody {
            email: self.email.to_string(),
            amount: self.amount,
        };
        let tx_body = serde_json::to_string(&body).expect("Error serializing body into JSON");
        let formatted_bearer = format!("Bearer {}", token.to_string());
        println!("Formatted header is: {:?}", formatted_bearer);
        let res = reqwest_client
            .post(format!("{}{}", default_url, endpoint))
            .header(AUTHORIZATION, formatted_bearer)
            .header(CONTENT_TYPE, "application/json".to_string())
            .body(tx_body)
            .send()
            .expect("Error initializing paystack request");

        match res.status() {
            StatusCode::UNAUTHORIZED => {
                println!("Oops! Unauthorized request. Please ensure you've set the correct headers")
            }
            _ => println!("Dunno... Looks Ok"),
        }
    }
}
#[derive(Default)]
pub struct Paystack {
    pub bearer_key: String,
    pub Transaction: Transaction,
}

impl Paystack {
    pub fn new(key: String) -> Paystack {
        Paystack {
            bearer_key: key,
            ..Default::default()
        }
    }
}
