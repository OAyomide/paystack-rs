use crate::{prelude::Currency, utils::*};
use reqwest::{
    blocking::{Client, Response},
    header::{AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use serde::Serialize;
use serde_json::to_string;
use std::fmt::Debug;

const DEDICATED_NUBAN_URL: &str = "https://api.paystack.co/dedicated_account";
#[derive(Debug, Serialize)]
pub struct DedicatedNuban {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct CreateDedicatedAccountBody<'a> {
    /// Customer ID or code
    pub customer: &'a str,
    /// The bank slug for preferred bank. To get a list of available banks, use the List Providers endpoint
    pub preferred_bank: Option<&'a str>,
    /// Subaccount code of the account you want to split the transaction with
    pub subaccount: Option<&'a str>,
    /// Split code consisting of the lists of accounts you want to split the transaction with
    pub split_code: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct ListDedicatedAccountsParams<'a> {
    /// Status of the dedicated account
    pub active: bool,
    /// Dedicated amount currency
    pub currency: Currency,
    /// The bank's slug in lowercase, without spaces e.g. wema-bank
    pub provider_slug: Option<&'a str>,
    /// The bank's ID e.g. 035
    pub bank_id: Option<&'a str>,
    /// The customer's ID
    pub customer: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct SplitDedicatedAccountTxBody<'a> {
    /// Customer ID or code
    pub customer: &'a str,
    /// Subaccount code of the account you want to split the transaction with
    pub subaccount: Option<&'a str>,
    /// Split code consisting of the lists of accounts you want to split the transaction with
    pub split_code: Option<&'a str>,
    /// The bank slug for preferred bank. To get a list of available banks, use the List Providers endpoint
    pub preferred_bank: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct RemoveSplitFromDedicatedAcctBody<'a> {
    pub account_number: &'a str,
}
/// The Dedicated NUBAN API enables Nigerian merchants manage unique payment accounts of their cuctomers.
impl DedicatedNuban {
    /// Create a Dedicated NUBAN and assign to a customer
    /// 💡 Bank Availability: Wema Bank is the only bank currently available
    pub fn create_dedicated_account(
        &self,
        body: CreateDedicatedAccountBody,
    ) -> Result<Response, String> {
        let res = make_post_request(
            self.bearer_auth.clone(),
            DEDICATED_NUBAN_URL.to_owned(),
            body,
        );
        return res;
    }

    pub fn list_dedicated_accounts(
        &self,
        params: Option<ListDedicatedAccountsParams>,
    ) -> Result<Response, String> {
        let res = make_get_request(
            self.bearer_auth.clone(),
            DEDICATED_NUBAN_URL.to_owned(),
            Some(params),
        );
        return res;
    }

    /// Get details of a dedicated account on your integration.
    pub fn fetch_dedicated_account(&self, id: &str) -> Result<Response, String> {
        let url = format!("{}/{}", DEDICATED_NUBAN_URL.to_owned(), id);
        let res = make_get_request(self.bearer_auth.clone(), url, None::<String>);
        return res;
    }

    /// Deactivate a dedicated account on your integration.
    pub fn deactivate_dedicated_account(&self, id: &str) -> Result<Response, String> {
        let url = format!("{}/{}", DEDICATED_NUBAN_URL.to_owned(), id);
        let res = make_delete_request(self.bearer_auth.clone(), url);
        return res;
    }

    /// Split a dedicated account transaction with one or more accounts
    pub fn split_dedicated_account_transaction(
        &self,
        body: SplitDedicatedAccountTxBody,
    ) -> Result<Response, String> {
        let res = make_post_request(
            self.bearer_auth.clone(),
            DEDICATED_NUBAN_URL.to_owned(),
            body,
        );
        return res;
    }

    pub fn remove_split_from_dedicated_acct(
        &self,
        body: RemoveSplitFromDedicatedAcctBody,
    ) -> Result<Response, String> {
        let reqwest_client = Client::new();
        let formatted_err_msg = format!(
            "[PAYSTACK ERROR]: Error making GET request to url: {}",
            DEDICATED_NUBAN_URL.to_owned()
        );
        let serialized_body =
            to_string(&body).expect("Error serializing RemoveSplitFromDedicatedAcctBody");
        let res = reqwest_client
            .delete(DEDICATED_NUBAN_URL.to_owned())
            .header(AUTHORIZATION, self.bearer_auth.clone())
            .header(CONTENT_TYPE, "application/json".to_string())
            .body(serialized_body)
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

    pub fn fetch_bank_providers(&self) -> Result<Response, String> {
        let url = format!("{}/available_providers", DEDICATED_NUBAN_URL.to_owned());
        let res = make_get_request(self.bearer_auth.clone(), url, None::<String>);
        return res;
    }
}
