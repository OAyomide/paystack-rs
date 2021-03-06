use crate::{prelude::Currency, utils::*};
use chrono::{prelude::DateTime, Utc};
use reqwest::blocking::Response;
use serde::Serialize;

/// The Refunds API allows you create and manage transaction refunds
#[derive(Default, Debug)]
pub struct Refunds {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct CreateRefundBody<'a> {
    /// Transaction reference or id
    pub transaction: &'a str,
    /// Amount is optional(defaults to original transaction amount) and cannot be more than the original transaction amount.
    pub amount: Option<&'a str>,
    /// Three-letter ISO currency.
    pub currency: Option<Currency>,
    /// Customer reason
    pub customer_note: Option<&'a str>,
    /// Merchant reason
    pub merchant_note: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct ListRefundsParams<'a> {
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    pub per_page: Option<i64>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i64>,
    /// A timestamp from which to start listing Refunds e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: Option<DateTime<Utc>>,
    /// A timestamp at which to stop listing Refunds e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: Option<DateTime<Utc>>,
    /// Transaction reference or id
    pub reference: Option<&'a str>,
    /// Three-letter ISO currency.
    pub currency: Option<Currency>,
}

const REFUND_URL: &str = "https://api.paystack.co/refund";
impl Refunds {
    /// Initiate a refund on your integration
    pub fn initiate_refund(&self, body: CreateRefundBody) -> Result<Response, String> {
        let res = make_request(&self.bearer_auth, REFUND_URL, Some(body), REQUEST::POST);
        return res;
    }

    /// List refunds available on your integration.
    pub fn list_refunds(&self, params: Option<ListRefundsParams>) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, REFUND_URL, params);
        return res;
    }
    /// Get details of a refund on your integration.
    /// takes a parameter reference. A transaction reference for the refund you want to fetch
    pub fn fetch_refund(&self, reference: &str) -> Result<Response, String> {
        let url = format!("{}/{}", REFUND_URL, reference);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }
}
