use crate::utils::*;
use chrono::{prelude::DateTime, Utc};
use reqwest::{
    blocking::{Client, Response},
    StatusCode,
};
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;
#[derive(Default, Debug)]
pub struct Customer {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct CreateCustomerBody<'a> {
    /// Customer's email address
    pub email: &'a str,
    /// Customer's first name
    pub first_name: &'a str,
    /// Customer's last name
    pub last_name: &'a str,
    /// Customer's phone number
    pub phone: Option<&'a str>,
    /// A set of key/value pairs that you can attach to the customer. It can be used to store additional information in a structured format.
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct ListCustomersParams {
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    pub per_page: Option<i64>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i64>,
    /// A timestamp from which to start listing customers e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: Option<DateTime<Utc>>,
    /// A timestamp at which to stop listing customers e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct UpdateCustomerBody<'a> {
    /// Customer's first name
    pub first_name: &'a str,
    /// Customer's last name
    pub last_name: &'a str,
    /// Customer's phone number
    pub phone: Option<&'a str>,
    /// A set of key/value pairs that you can attach to the customer. It can be used to store additional information in a structured format.
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct ValidateCustomerBody<'a> {
    /// Customer's first name
    pub first_name: &'a str,
    /// Customer's last name
    pub last_name: &'a str,
    /// Predefined types of identification. Valid values: bvn
    pub tx_type: &'a str,
    /// Customer's identification number
    pub value: &'a str,
    /// 2 letter country code of identification issuer
    pub country: &'a str,
}

#[derive(Debug, Serialize)]
pub enum RiskAction {
    Default,
    Allow,
    Deny,
}

// impl Default for RiskAction {
//     fn default() -> Self {
//         RiskAction::Default
//     }
// }

#[derive(Debug, Serialize)]
pub struct WhitelistOrBlacklistCustomerBody<'a> {
    /// Customer's code, or email address
    pub customer: &'a str,
    /// One of the possible risk actions [ default, allow, deny ]. allow to whitelist. deny to blacklist. Customers start with a default risk action.
    pub risk_action: Option<RiskAction>,
}

#[derive(Debug, Serialize)]
pub struct DeactivateAuthorizationBody<'a> {
    /// Authorization code to be deactivated
    pub authorization_code: &'a str,
}

const CUSTOMER_URL: &str = "https://api.paystack.co/customer";
impl Customer {
    /// Create a customer on your integration
    pub fn create_customer(&self, body: CreateCustomerBody) -> Result<Response, String> {
        let res = make_post_request(self.bearer_auth.clone(), CUSTOMER_URL.to_owned(), body);
        return res;
    }

    /// List customers available on your integration.
    pub fn list_customers(&self, params: ListCustomersParams) -> Result<Response, String> {
        let reqwest_client = Client::new();
        let res = reqwest_client
            .get(CUSTOMER_URL.to_owned())
            .query(&[
                ("perPage", params.per_page.unwrap()),
                ("page", params.page.unwrap()),
            ])
            .query(&[("from", params.from.unwrap()), ("to", params.to.unwrap())])
            .send()
            .expect("Error listing all customers");

        match res.status() {
            StatusCode::OK => return Ok(res),
            StatusCode::BAD_REQUEST => return Err("Bad request. Please check the body".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR => {
                return Err("An error occured on the paystack server: please try again".to_string())
            }
            _ => return Ok(res),
        }
    }
    /// Get details of a customer on your integration.
    /// takes a parameter emai_or_code. An email or customer code for the customer you want to fetch
    pub fn fetch_customer(&self, email_or_code: &str) -> Result<Response, String> {
        let url = format!("{}/{}", CUSTOMER_URL.to_owned(), email_or_code);
        let res = make_get_request(self.bearer_auth.clone(), url);
        return res;
    }

    pub fn update_customer(
        &self,
        code: &str,
        body: UpdateCustomerBody,
    ) -> Result<Response, String> {
        let url = format!("{}/{}", CUSTOMER_URL.to_owned(), code);
        let res = make_post_request(self.bearer_auth.clone(), url, body);
        return res;
    }

    pub fn validate_customer(
        &self,
        code: &str,
        body: ValidateCustomerBody,
    ) -> Result<Response, String> {
        let url = format!("{}/{}/identification", CUSTOMER_URL.to_owned(), code);
        let res = make_post_request(self.bearer_auth.clone(), url, body);
        return res;
    }

    /// Whitelist or blacklist a customer on your integration
    pub fn whitelist_or_blacklist_customer(
        &self,
        body: WhitelistOrBlacklistCustomerBody,
    ) -> Result<Response, String> {
        let url = format!("{}/set_risk_action", CUSTOMER_URL.to_owned());
        let res = make_post_request(self.bearer_auth.clone(), url, body);
        return res;
    }

    /// Deactivate an authorization when the card needs to be forgotten
    pub fn deactivate_authorization(
        &self,
        body: DeactivateAuthorizationBody,
    ) -> Result<Response, String> {
        let url = format!("{}/deactivate_authorization", CUSTOMER_URL.to_owned());
        let res = make_post_request(self.bearer_auth.clone(), url, body);
        return res;
    }
}
