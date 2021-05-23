use chrono::{DateTime, Local};
use reqwest::blocking::Response;
use serde::Serialize;

use crate::utils::{make_get_request, make_request, REQUEST};

/// The Disputes API allows you manage transaction disputes on your integration
#[derive(Debug, Default)]
pub struct Disputes {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DisputeStatus {
    AwaitingMerchantFeedback,
    AwaitingBankFeedback,
    Pending,
    Resolved,
    Declined,
    MerchantAccepted,
}
#[derive(Debug, Serialize)]
pub struct ListDisputesParams<'a> {
    /// A timestamp from which to start listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: DateTime<Local>,
    /// A timestamp at which to stop listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: DateTime<Local>,
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    #[serde(rename = "perPage")]
    pub per_page: Option<i64>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i128>,
    pub transaction: &'a str,
    /// Dispute Status. Acceptable values: `{ awaiting-merchant-feedback | awaiting-bank-feedback | pending | resolved }`
    pub status: DisputeStatus,
}

#[derive(Debug, Serialize)]
pub struct UpdateDisputeBody<'a> {
    /// the amount to refund, in **kobo** if currency is `NGN`, **pesewas**, if currency is `GHS`, and **cents**, if currency is `ZAR`
    pub refund_amount: i64,
    // TODO: link to upload dispute url
    /// filename of attachment returned via response from upload url(GET /dispute/:id/upload_url)
    pub uploaded_filename: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct AddEvidenceBody<'a> {
    /// Customer email
    pub customer_email: &'a str,
    /// Customer name
    pub customer_name: &'a str,
    /// Customer phone
    pub customer_phone: &'a str,
    /// Details of service involved
    pub service_details: &'a str,
    /// Delivery Address
    pub delivery_address: Option<&'a str>,
    /// ISO 8601 representation of delivery date (YYYY-MM-DD)
    pub delivery_date: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize)]
pub struct GetUploadURLParams<'a> {
    /// The file name, with its extension, that you want to upload. e.g `filename.pdf`
    pub upload_filename: &'a str,
}

#[derive(Debug, Serialize)]
pub struct ResolveDisputeBody<'a> {
    /// Dispute resolution. Accepted values: { merchant-accepted | declined }.
    pub resolution: &'a str,
    /// Reason for resolving
    pub message: &'a str,
    /// the amount to refund, in **kobo** if currency is `NGN`, **pesewas**, if currency is `GHS`, and **cents**, if currency is `ZAR`
    pub refund_amount: i64,
    /// filename of attachment returned via response from upload url(GET /dispute/:id/upload_url)
    pub uploaded_filename: &'a str,
    /// Evidence Id for fraud claims
    pub evidence: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ExportDisputesBody<'a> {
    /// A timestamp from which to start listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: DateTime<Local>,
    /// A timestamp at which to stop listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: DateTime<Local>,
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    #[serde(rename = "perPage")]
    pub per_page: Option<i64>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i128>,
    pub transaction: &'a str,
    /// Dispute Status. Acceptable values: `{ awaiting-merchant-feedback | awaiting-bank-feedback | pending | resolved }`
    pub status: DisputeStatus,
}

const DISPUTE_URL: &str = "https://api.paystack.co/dispute";
impl Disputes {
    /// List disputes filed against you
    pub fn list_disputes(&self, params: ListDisputesParams) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, DISPUTE_URL, Some(params));
        return res;
    }

    /// Get more details about a dispute.
    /// - id: The dispute `ID` you want to fetch
    pub fn fetch_dispute(&self, id: &str) -> Result<Response, String> {
        let url = format!("{}/{}", DISPUTE_URL, id);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Get more details about a dispute.
    ///  - id: The transaction `ID` you want to fetch
    pub fn list_transaction_disputes(&self, id: &str) -> Result<Response, String> {
        let url = format!("{}/transaction/{}", DISPUTE_URL, id);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Update details of a dispute on your integration
    pub fn update_dispute(&self, id: &str, body: UpdateDisputeBody) -> Result<Response, String> {
        let url = format!("{}/{}", DISPUTE_URL, id);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::PUT);
        return res;
    }

    /// Provide evidence for a dispute
    pub fn add_evidence(&self, id: &str, body: AddEvidenceBody) -> Result<Response, String> {
        let url = format!("{}/{}/dispute", DISPUTE_URL, id);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// Resolve a dispute on your integration
    pub fn get_upload_url(&self, id: &str, params: GetUploadURLParams) -> Result<Response, String> {
        let url = format!("{}/{}/upload_url", DISPUTE_URL, id);
        let res = make_get_request(&self.bearer_auth, &url, Some(params));
        return res;
    }

    /// Resolve a dispute on your integration
    pub fn resolve_dispute(&self, id: &str, body: ResolveDisputeBody) -> Result<Response, String> {
        let url = format!("{}/{}/resolve", DISPUTE_URL, id);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::PUT);
        return res;
    }

    /// Export disputes available on your integration
    pub fn export_disputes(&self, params: ExportDisputesBody) -> Result<Response, String> {
        let url = format!("{}/export", DISPUTE_URL);
        let res = make_get_request(&self.bearer_auth, &url, Some(params));
        return res;
    }
}
