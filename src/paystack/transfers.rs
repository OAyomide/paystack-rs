use chrono::{DateTime, Local};
use reqwest::blocking::Response;
use serde::Serialize;
use serde_json::Value as JSON;

use crate::{
    prelude::Currency,
    utils::{make_get_request, make_request, REQUEST},
};

const TRANSFERS_URL: &str = "https://api.paystack.co/transfer";
/// The Transfers API allows you automate sending money on your integration
/// - 💡 Feature Availability
/// This feature is only available to businesses in Nigeria and Ghana.
#[derive(Debug, Default)]
pub struct Transfers {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct InitiateTransferBody<'a> {
    /// Where should we transfer from. Only `balance` for now
    pub source: &'a str,
    /// Amount to transfer in kobo if currency is NGN and pesewas if currency is GHS.
    pub amount: i64,
    /// Code for transfer recipient
    pub recipient: &'a str,
    /// The reason for the transfer
    pub reason: Option<&'a str>,
    /// Specify the currency of the transfer. Defaults to NGN
    pub currency: Option<Currency>,
    /// If specified, the field should be a unique identifier (in lowercase) for the object.
    /// Only -,_ and alphanumeric characters allowed.
    pub reference: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct InitiateBulkTransferBody<'a> {
    /// The transfer code you want to finalize
    pub source: &'a str,
    /// A list of transfer object. Each object should contain `amount`, `recipient`, and `reference`
    pub transfers: Vec<JSON>,
}

#[derive(Debug, Serialize)]
pub struct FinalizeTransferBody<'a> {
    /// The transfer code you want to finalize
    pub transfer_code: &'a str,
    /// OTP sent to business phone to verify transfer
    pub otp: &'a str,
}

#[derive(Debug, Serialize)]
pub struct ListTransfersParams<'a> {
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    #[serde(rename = "perPage")]
    pub per_page: Option<i128>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i128>,
    /// Filter by customer ID.
    pub customer: &'a str,
    /// A timestamp from which to start listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: Option<DateTime<Local>>,
    /// A timestamp at which to stop listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: Option<DateTime<Local>>,
}

impl Transfers {
    /// Status of transfer object returned will be `pending` if OTP is disabled.
    /// In the event that an OTP is required, status will read `otp`.
    pub fn initiate_transfers(&self, body: InitiateTransferBody) -> Result<Response, String> {
        let res = make_request(&self.bearer_auth, TRANSFERS_URL, Some(body), REQUEST::POST);
        return res;
    }

    /// Finalize an initiated transfer
    pub fn finalize_transfer(&self, body: FinalizeTransferBody) -> Result<Response, String> {
        let url = format!("{}/finalize_transfer", TRANSFERS_URL);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// You need to disable the Transfers OTP requirement to use this endpoint.
    pub fn initiate_bulk_transfer(
        &self,
        body: InitiateBulkTransferBody,
    ) -> Result<Response, String> {
        let url = format!("{}/bulk", TRANSFERS_URL);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// List the transfers made on your integration.
    pub fn list_transfers(&self, params: ListTransfersParams) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, TRANSFERS_URL, Some(params));
        return res;
    }

    /// Get details of a transfer on your integration.
    pub fn fetch_transfer(&self, id_or_code: &str) -> Result<Response, String> {
        let url = format!("{}/{}", TRANSFERS_URL, id_or_code);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Verify the status of a transfer on your integration.
    pub fn verify_transfer(&self, reference: &str) -> Result<Response, String> {
        let url = format!("{}/verify/{}", TRANSFERS_URL, reference);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }
}
