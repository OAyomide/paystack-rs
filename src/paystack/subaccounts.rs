use crate::utils::*;
use chrono::{DateTime, Utc};
use reqwest::blocking::Response;
use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug};

const SUBACCOUNT_URL: &str = "https://api.paystack.co/subaccount";

#[derive(Debug, Default)]
pub struct Subaccount {
    pub(crate) bearer_auth: String,
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
impl Subaccount {
    /// Create a subacount on your integration
    pub fn create_subaccount(&self, body: CreateSubaccountBody) -> Result<Response, String> {
        let res = make_request(
            self.bearer_auth.to_owned(),
            SUBACCOUNT_URL.to_owned(),
            Some(body),
            REQUEST::POST,
        );
        return res;
    }

    /// List subaccounts available on your integration.
    pub fn list_subaccounts<T>(&self, params: Option<T>) -> Result<Response, String>
    where
        T: Debug + Serialize,
    {
        let res = make_get_request(
            self.bearer_auth.to_owned(),
            SUBACCOUNT_URL.to_owned(),
            params,
        );
        return res;
    }

    /// Get details of a subaccount on your integration.
    pub fn fetch_subaccount(&self, id: &str) -> Result<Response, String> {
        let url = format!("{}/{}", SUBACCOUNT_URL, id);
        let res = make_get_request(
            self.bearer_auth.to_owned(),
            url,
            None::<HashMap<String, String>>,
        );
        return res;
    }

    pub fn update_subaccount(
        &self,
        id: &str,
        body: UpdateSubaccountBody,
    ) -> Result<Response, String> {
        let url = format!("{}/{}", SUBACCOUNT_URL, id);
        let res = make_request(self.bearer_auth.to_owned(), url, Some(body), REQUEST::PUT);
        return res;
    }
}
