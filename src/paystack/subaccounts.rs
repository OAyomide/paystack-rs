use crate::{prelude::Currency, utils::*};
use chrono::{DateTime, Utc};
use reqwest::{
    blocking::{Client, Response},
    header::{AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use serde::Serialize;
use serde_json::{to_string, Value};
use std::fmt::Debug;

const SUBACCOUNT_URL: &str = "https://api.paystack.co/subaccount";

#[derive(Debug)]
pub struct Subaccount<'a> {
    pub(crate) bearer_auth: &'a str,
}

#[derive(Debug, Serialize)]
pub struct CreateSubaccountBody<'a> {
    /// Name of business for subaccount
    pub business_name: &'a str,
    /// Bank Code for the bank. You can get the list of Bank Codes by calling the List Banks endpoint.
    pub settlement_bank: &'a str,
    /// Bank Account Number
    pub account_number: &'a str,
    /// The default percentage charged when receiving on behalf of this subaccount
    pub percentage_charge: f64,
    /// A description for this subaccount
    pub description: &'a str,
    /// A contact email for the subaccount
    pub primary_contact_email: Option<&'a str>,
    /// A name for the contact person for this subaccount
    pub primary_contact_name: Option<&'a str>,
    /// A phone number to call for this subaccount
    pub primary_contact_phone: Option<&'a str>,
    /// Stringified JSON object. Add a custom_fields attribute which has an array of objects if you would like the fields to be added to your transaction when displayed on the dashboard. Sample: {"custom_fields":[{"display_name":"Cart ID","variable_name": "cart_id","value": "8393"}]}
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct ListSubaccountParams {
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    pub per_page: Option<i64>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i64>,
    /// A timestamp from which to start listing subaccounts e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: Option<DateTime<Utc>>,
    /// A timestamp at which to stop listing subaccounts e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct UpdateSubaccountBody<'a> {
    /// Name of business for subaccount
    pub business_name: &'a str,
    /// Bank Code for the bank. You can get the list of Bank Codes by calling the List Banks endpoint.
    pub settlement_bank: &'a str,
    /// Bank Account Number
    pub account_number: Option<&'a str>,
    /// The default percentage charged when receiving on behalf of this subaccount
    pub percentage_charge: Option<f64>,
    /// A description for this subaccount
    pub description: Option<&'a str>,
    /// A contact email for the subaccount
    pub primary_contact_email: Option<&'a str>,
    /// A name for the contact person for this subaccount
    pub primary_contact_name: Option<&'a str>,
    /// A phone number to call for this subaccount
    pub primary_contact_phone: Option<&'a str>,
    /// Stringified JSON object. Add a custom_fields attribute which has an array of objects if you would like the fields to be added to your transaction when displayed on the dashboard. Sample: {"custom_fields":[{"display_name":"Cart ID","variable_name": "cart_id","value": "8393"}]}
    pub metadata: Option<Value>,
}

/// The Subaccounts API allows you create and manage subaccounts on your integration. Subaccounts can be used to split payment between two accounts (your main account and a sub account)
impl Subaccount<'_> {
    /// Create a subacount on your integration
    pub fn create_subaccount(&self, body: CreateSubaccountBody) -> Result<Response, String> {
        let res = make_post_request(self.bearer_auth.to_owned(), SUBACCOUNT_URL.to_owned(), body);
        return res;
    }

    /// List subaccounts available on your integration.
    pub fn list_subaccounts(
        &self,
        params: Option<ListSubaccountParams>,
    ) -> Result<Response, String> {
        let reqwest_client = Client::new();
        let formatted_err_msg = format!(
            "[PAYSTACK ERROR]: Error making GET request to url: {}",
            SUBACCOUNT_URL
        );
        let params = params.expect("Error unwrapping params");
        let res = reqwest_client
            .get(SUBACCOUNT_URL)
            .header(AUTHORIZATION, self.bearer_auth)
            .query(&[
                ("perPage", params.per_page.unwrap()),
                ("page", params.page.unwrap()),
            ])
            .query(&[("from", params.from.unwrap()), ("to", params.to.unwrap())])
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

    /// Get details of a subaccount on your integration.
    pub fn fetch_subaccount(&self, id: &str) -> Result<Response, String> {
        let url = format!("{}/{}", SUBACCOUNT_URL, id);
        let res = make_get_request(self.bearer_auth.to_owned(), url, None::<String>);
        return res;
    }

    pub fn update_subaccount(
        &self,
        id: &str,
        body: UpdateSubaccountBody,
    ) -> Result<Response, String> {
        let url = format!("{}/{}", SUBACCOUNT_URL, id);
        let res = make_put_request(self.bearer_auth.to_owned(), url, body);
        return res;
    }
}
