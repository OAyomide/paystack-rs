use crate::utils::make_get_request;
use chrono::{DateTime, Local};
use reqwest::blocking::Response;
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct FetchSettlementsBody<'a> {
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    #[serde(rename = "perPage")]
    pub per_page: Option<i128>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i128>,
    /// Provide a subaccount ID to export only settlements for that subaccount. Set to none to export only transactions for the account.
    pub subacount: Option<&'a str>,
    /// A timestamp from which to start listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: Option<DateTime<Local>>,
    /// A timestamp at which to stop listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize)]
pub struct FetchSettleTxBody<'a> {
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    #[serde(rename = "perPage")]
    pub per_page: Option<i128>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i128>,
    /// Provide a subaccount ID to export only settlements for that subaccount. Set to none to export only transactions for the account.
    pub subacount: Option<&'a str>,
    /// A timestamp from which to start listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: Option<DateTime<Local>>,
    /// A timestamp at which to stop listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: Option<DateTime<Local>>,
}
/// The Settlements API allows you gain insights into payouts made by Paystack to your bank account
#[derive(Debug, Default)]
pub struct Settlements {
    pub(crate) bearer_auth: String,
}

const SETTLEMENTS_URL: &str = "https://api.paystack.co/settlement";
impl Settlements {
    /// Fetch settlements made to your settlement accounts.
    pub fn fetch_settlements(
        &self,
        params: Option<FetchSettlementsBody>,
    ) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, SETTLEMENTS_URL, Some(params));
        return res;
    }

    /// Get the transactions that make up a particular settlement
    pub fn fetch_settlement_transactions(
        &self,
        id: &str,
        params: Option<FetchSettleTxBody>,
    ) -> Result<Response, String> {
        let url = format!("{}/{}/transactions", SETTLEMENTS_URL, id);
        let res = make_get_request(&self.bearer_auth, &url, params);
        return res;
    }
}
