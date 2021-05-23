use chrono::{DateTime, Local};
use reqwest::blocking::Response;
use serde::Serialize;
use serde_json::Value as JSON;

use crate::{
    prelude::Currency,
    utils::{make_get_request, make_request, REQUEST},
};

const TRANSFER_RECIPIENT_URL: &str = "https://api.paystack.co/transferrecipient";
/// The Transfer Recipients API allows you create and manage beneficiaries that you send money to
///  
/// ```text
///  - 💡 Feature Availability
/// This feature is only available to businesses in Nigeria and Ghana.
/// ```
#[derive(Debug, Default)]
pub struct TransferRecipients {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTransferRecipientBody<'a> {
    #[serde(rename = "type")]
    /// Recipient Type (Only `nuban` at this time)
    pub recipient_type: &'a str,
    /// A name for the recipient
    pub name: &'a str,
    /// Required if type is `nuban`
    pub account_number: &'a str,
    // TODO: link to module that implements list of Bank codes
    /// Required if type is nuban. You can get the [CreatePaymentPagesBody][list of Bank Codes] by calling the List Banks endpoint.
    pub bank_code: &'a str,
    /// A description for this plan
    pub description: Option<&'a str>,
    /// Currency for the account receiving the transfer
    pub currency: Option<Currency>,
    /// An authorization code from a previous transaction
    pub authorization_code: Option<&'a str>,
    /// Store additional information about your recipient in a structured format, JSON
    pub metadata: Option<JSON>,
}

/// A list of transfer recipient object. Each object should contain `type`, `name`, and `bank_code`.
/// Any [create_transfer_recipient][Create Transfer Recipient] param can also be passed.
#[derive(Debug, Serialize)]
pub struct BulkCreateTransferRecipient {
    pub batch: Vec<JSON>,
}

#[derive(Debug, Serialize)]
pub struct ListTransferRecipientsParams {
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    #[serde(rename = "perPage")]
    pub per_page: Option<i128>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i128>,
    /// A timestamp from which to start listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: Option<DateTime<Local>>,
    /// A timestamp at which to stop listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize)]
pub struct UpdateTransferRecipient<'a> {
    /// A name for the recipient
    pub name: &'a str,
    /// Email address of the recipient
    pub email: &'a str,
    /// A description for this plan
    pub description: &'a str,
}
impl TransferRecipients {
    /// Creates a new recipient. A duplicate account number will lead to the retrieval of the existing record.
    pub fn create_transfer_recipient(
        &self,
        body: CreateTransferRecipientBody,
    ) -> Result<Response, String> {
        let res = make_request(
            &self.bearer_auth,
            TRANSFER_RECIPIENT_URL,
            Some(body),
            REQUEST::POST,
        );
        return res;
    }

    ///Create multiple transfer recipients in batches. A duplicate account number will lead to the retrieval of the existing record.
    pub fn bulk_create_transfer_recipient(
        &self,
        body: BulkCreateTransferRecipient,
    ) -> Result<Response, String> {
        let url = format!("{}/bulk", TRANSFER_RECIPIENT_URL);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// List transfer recipients available on your integration
    pub fn list_transfer_recipients(
        &self,
        params: ListTransferRecipientsParams,
    ) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, TRANSFER_RECIPIENT_URL, Some(params));
        return res;
    }

    /// Fetch the details of a transfer recipient
    ///  - id_or_code: An ID or code for the recipient whose details you want to receive.
    pub fn fetch_transfer_recipient(&self, id_or_code: &str) -> Result<Response, String> {
        let url = format!("{}/{}", TRANSFER_RECIPIENT_URL, id_or_code);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Update an existing recipient. An duplicate account number will lead to the retrieval of the existing record.
    pub fn update_transfer_recipient(
        &self,
        body: UpdateTransferRecipient,
        id_or_code: &str,
    ) -> Result<Response, String> {
        let url = format!("{}/{}", TRANSFER_RECIPIENT_URL, id_or_code);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::PUT);
        return res;
    }

    /// Deletes a transfer recipient (sets the transfer recipient to inactive)
    pub fn delete_transfer_recipient(&self, id_or_code: &str) -> Result<Response, String> {
        let url = format!("{}/{}", TRANSFER_RECIPIENT_URL, id_or_code);
        let res = make_request(&self.bearer_auth, &url, None::<String>, REQUEST::DELETE);
        return res;
    }
}
