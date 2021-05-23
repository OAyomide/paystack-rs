use crate::{
    prelude::{Currency, Status},
    utils::{make_get_request, make_request, REQUEST},
};
use chrono::{DateTime, Local};
use reqwest::blocking::Response;
use serde::Serialize;
use serde_json::Value as JSON;

#[derive(Debug, Default)]
/// The Invoices API allows you issue out and manage payment requests
pub struct Invoices {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct CreateInvoiceBody<'a> {
    /// Customer id or code
    pub customer: &'a str,
    /// Payment request amount. Only useful if line items and tax values are ignored. endpoint will throw a friendly warning if neither is available.
    pub amount: i64,
    /// ISO 8601 representation of request due date
    pub due_date: DateTime<Local>,
    /// A short description of the payment request
    pub description: Option<&'a str>,
    /// Array of line items int the format [{"name":"item 1", "amount":2000}]
    pub line_items: Option<Vec<JSON>>,
    /// Array of taxes to be charged in the format [{"name":"VAT", "amount":2000}]
    pub tax: Option<Vec<JSON>>,
    /// Specify the currency of the invoice. Allowed values are NGN, GHS, ZAR and USD. Defaults to NGN
    pub currency: Option<Currency>,
    /// Indicates whether Paystack sends an email notification to customer. Defaults to true
    pub send_notification: Option<bool>,
    /// Indicate if request should be saved as draft. Defaults to false and overrides send_notification
    pub draft: Option<bool>,
    /// Set to true to create a draft invoice (adds an auto incrementing invoice number if none is provided) even if there are no line_items or tax passed
    pub has_invoice: Option<bool>,
    /// Numeric value of invoice. Invoice will start from 1 and auto increment from there. This field is to help override whatever value Paystack decides. Auto increment for subsequent invoices continue from this point.
    pub invoice_number: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListInvoicesParams<'a> {
    #[serde(rename = "per_page")]
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    pub per_page: Option<i64>,
    /// Specify exactly what invoice you want to page. If not specify we use a default value of 1.
    pub page: Option<i64>,
    /// Filter by customer ID
    pub customer: &'a str,
    /// Filter by invoice status
    pub status: Status,
    /// Filter by currency. Allowed values are `NGN`, `GHS`, `ZAR` and `USD`.
    pub currency: Currency,
    /// Show archived invoices
    pub include_archive: &'a str,
    /// A timestamp from which to start listing invoice e.g. `2016-09-24T00:00:05.000Z`, `2016-09-21`
    pub from: Option<DateTime<Local>>,
    /// A timestamp at which to stop listing invoice e.g. `2016-09-24T00:00:05.000Z`, `2016-09-21`
    pub to: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize)]
pub struct UpdateInvoiceBody<'a> {
    /// Customer id or code
    pub customer: &'a str,
    /// Payment request amount. Only useful if line items and tax values are ignored. endpoint will throw a friendly warning if neither is available.
    pub amount: i64,
    /// ISO 8601 representation of request due date
    pub due_date: Option<DateTime<Local>>,
    /// A short description of the payment request
    pub description: Option<&'a str>,
    /// Array of line items int the format [{"name":"item 1", "amount":2000}]
    pub line_items: Option<Vec<JSON>>,
    /// Array of taxes to be charged in the format [{"name":"VAT", "amount":2000}]
    pub tax: Option<Vec<JSON>>,
    /// Specify the currency of the invoice. Allowed values are NGN, GHS, ZAR and USD. Defaults to NGN
    pub currency: Option<Currency>,
    /// Indicates whether Paystack sends an email notification to customer. Defaults to true
    pub send_notification: Option<bool>,
    /// Indicate if request should be saved as draft. Defaults to false and overrides send_notification
    pub draft: Option<bool>,
    /// Numeric value of invoice. Invoice will start from 1 and auto increment from there.
    /// This field is to help override whatever value Paystack decides.
    /// Auto increment for subsequent invoices continue from this point.
    pub invoice_number: Option<i64>,
}
const INVOICES_URL: &str = "https://api.paystack.com/paymentrequest";
impl Invoices {
    /// Create an invoice for payment on your integration
    pub fn create_invoice(&self, body: CreateInvoiceBody) -> Result<Response, String> {
        let res = make_request(&self.bearer_auth, INVOICES_URL, Some(body), REQUEST::POST);
        return res;
    }

    /// List the invoice available on your integration.
    pub fn list_invoices(&self, params: Option<ListInvoicesParams>) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, INVOICES_URL, params);
        return res;
    }

    /// Get details of an invoice on your integration.
    /// - id_or_code: Invoice ID or slug
    pub fn view_invoice(&self, id_or_code: &str) -> Result<Response, String> {
        let url = format!("{}/{}", INVOICES_URL, id_or_code);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Verify details of an invoice on your integration.
    /// - id_or_code: Invoice ID or slug
    pub fn verify_invoice(&self, id_or_code: &str) -> Result<Response, String> {
        let url = format!("{}/verify/{}", INVOICES_URL, id_or_code);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Send notification of an invoice to your customers
    /// - id_or_code: Invoice ID or slug
    pub fn send_notification(&self, id_or_code: &str) -> Result<Response, String> {
        let url = format!("{}/notify/{}", INVOICES_URL, id_or_code);
        let res = make_request(&self.bearer_auth, &url, None::<String>, REQUEST::POST);
        return res;
    }

    /// Get invoice metrics for dashboard
    pub fn invoice_total(&self) -> Result<Response, String> {
        let url = format!("{}/totals", INVOICES_URL);
        let res = make_request(&self.bearer_auth, &url, None::<String>, REQUEST::POST);
        return res;
    }

    /// Finalize a Draft Invoice
    /// - id_or_code: Invoice ID or slug
    pub fn finalize_invoice(&self, id_or_code: &str) -> Result<Response, String> {
        let url = format!("{}/finalize/{}", INVOICES_URL, id_or_code);
        let res = make_request(&self.bearer_auth, &url, None::<String>, REQUEST::POST);
        return res;
    }

    /// Update an invoice details on your integration
    /// - id_or_code: Invoice ID or slug
    pub fn update_invoice(
        &self,
        id_or_code: &str,
        body: UpdateInvoiceBody,
    ) -> Result<Response, String> {
        let url = format!("{}/{}", INVOICES_URL, id_or_code);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::PUT);
        return res;
    }

    /// Used to archive an invoice. Invoice will no longer be fetched on list or returned on verify.
    /// - id_or_code: Invoice ID or slug
    pub fn archive_invoice(&self, id_or_code: &str) -> Result<Response, String> {
        let url = format!("{}/{}", INVOICES_URL, id_or_code);
        let res = make_request(&self.bearer_auth, &url, None::<String>, REQUEST::POST);
        return res;
    }
}
