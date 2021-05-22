use crate::prelude::Currency;
use crate::utils::*;
use reqwest::blocking::Response;
use serde::Serialize;

use crate::utils::make_request;

#[derive(Debug, Default)]
pub struct Plans {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Interval {
    Daily,
    Weekly,
    Monthly,
    Biannual,
    Annually,
}

#[derive(Debug, Serialize)]
pub struct CreatePlan {
    /// Name of plan
    pub name: String,
    /// Amount should be in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR
    pub amount: i128,
    /// Interval in words. Valid intervals are: daily, weekly, monthly,biannually, annually.
    pub interval: Interval,
    /// A description for this plan
    pub description: Option<String>,
    /// Set to false if you don't want invoices to be sent to your customers
    pub send_invoices: Option<bool>,
    /// Set to false if you don't want text messages to be sent to your customers
    pub send_sms: Option<String>, // TODO: change this to bool and see if it'll work.
    /// Currency in which amount is set. Allowed values are NGN, GHS, ZAR or USD
    pub currency: Option<Currency>,
    /// Number of invoices to raise during subscription to this plan. Can be overridden by specifying an invoice_limit while subscribing.
    pub invoice_limit: Option<i128>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPlansParams {
    per_page: Option<i128>,
    page: Option<i128>,
    interval: Option<i128>,
    amount: Option<Currency>,
}

#[derive(Debug, Serialize)]
pub struct UpdatePlanBody {
    /// Name of plan
    pub name: String,
    /// Amount should be in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR
    pub amount: i128,
    /// Interval in words. Valid intervals are: daily, weekly, monthly,biannually, annually.
    pub interval: Interval,
    /// A description for this plan
    pub description: Option<String>,
    /// Set to false if you don't want invoices to be sent to your customers
    pub send_invoices: Option<bool>,
    /// Set to false if you don't want text messages to be sent to your customers
    pub send_sms: Option<String>, // TODO: change this to bool and see if it'll work.
    /// Currency in which amount is set. Allowed values are NGN, GHS, ZAR or USD
    pub currency: Option<Currency>,
    /// Number of invoices to raise during subscription to this plan. Can be overridden by specifying an invoice_limit while subscribing.
    pub invoice_limit: Option<i128>,
}
const PLANS_URL: &str = "https://api.paystack.co/plan";
/// The Plans API allows you create and manage installment payment options on your integration
impl Plans {
    /// Create a plan on your integration
    pub fn create_plan(&self, body: CreatePlan) -> Result<Response, String> {
        let res = make_request(&self.bearer_auth, PLANS_URL, Some(body), REQUEST::POST);
        return res;
    }

    /// List plans available on your integration.
    pub fn list_plans(&self, params: Option<ListPlansParams>) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, PLANS_URL, params);
        return res;
    }

    /// Get details of a plan on your integration.
    /// id_or_code: The plan ID or code you want to fetch
    pub fn fetch_plan(&self, id_or_code: String) -> Result<Response, String> {
        let url = format!("{}/{}", PLANS_URL, id_or_code);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Update a plan details on your integration
    pub fn update_plan(
        &self,
        id_or_code: String,
        body: UpdatePlanBody,
    ) -> Result<Response, String> {
        let url = format!("{}/{}", PLANS_URL, id_or_code);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::PUT);
        return res;
    }
}
