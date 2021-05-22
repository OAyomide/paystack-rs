use std::fmt::Debug;

use crate::{prelude::Currency, utils::*};
use chrono::{DateTime, Utc};
use reqwest::blocking::Response;
use serde::Serialize;

const SPLIT_PAYMENT_URL: &str = "https://api.paystack.co/split";
#[derive(Default, Debug)]
/// The Transaction Splits API enables merchants split the settlement for a transaction across their payout account, and one or more Subaccounts.
pub struct TransactionSplit {
    pub(crate) bearer_auth: String,
}

#[derive(Serialize, Debug)]
pub struct SubaccountsBody<'a> {
    pub subaccount_code: &'a str,
    pub share: i64,
}

#[derive(Serialize, Debug)]
pub enum BearerType {
    Subaccount,
    Account,
    Allproportional,
    All,
}

#[derive(Debug, Serialize)]
pub enum TxType {
    Percentage,
    Flat,
}

#[derive(Serialize, Debug)]
pub struct CreateSplitPaymentBody<'a> {
    /// Name of the transaction split
    pub name: &'a str,

    // cannot use type because type is a reserved keyword in rust
    /// The type of transaction split you want to create. You can use one of the following: percentage | flat
    pub tx_type: TxType,
    /// Any of NGN, GHS, ZAR, or USD
    pub currency: Currency,
    /// A list of object containing subaccount code and number of shares: [{subaccount_code: ‘ACT_xxxxxxxxxx’, share: xxx},{...}]
    pub subaccount: Vec<SubaccountsBody<'a>>,
    /// Any of subaccount | account | all-proportional | all
    pub bearer_type: BearerType,
    /// Subaccount code
    pub bearer_subaccount: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOrSearchSplitsParams<'a> {
    /// The name of the split
    pub name: &'a str,
    ///Any of true or false
    pub active: bool,
    ///Sort by name, defaults to createdAt date
    pub sorted_by: Option<&'a str>,
    /// Number of splits per page. If not specify we use a default value of 50.
    pub per_page: Option<i64>,
    /// Page number to view. If not specify we use a default value of 1.
    pub page: Option<i64>,
    /// A timestamp from which to start listing splits e.g. 2019-09-24T00:00:05.000Z, 2019-09-21
    pub from: Option<DateTime<Utc>>,
    /// A timestamp at which to stop listing splits e.g. 2019-09-24T00:00:05.000Z, 2019-09-21
    pub to: Option<DateTime<Utc>>,
}

#[derive(Serialize, Debug)]
pub struct UpdateSplitBody<'a> {
    /// Name of the transaction split
    pub name: &'a str,
    /// True or False
    pub active: bool,
    /// Any of the following values: subaccount | account | all-proportional | all
    pub bearer_type: BearerType,
    /// Subaccount code of a subaccount in the split group. This should be specified only if the bearer_type is subaccount
    pub bearer_subaccount: &'a str,
}

#[derive(Debug, Serialize)]
pub struct AddOrUpdateSplitSubaccountBody<'a> {
    /// This is the sub account code
    pub subaccount: &'a str,
    /// This is the transaction share for the subaccount
    pub share: i64,
}

#[derive(Debug, Serialize)]
pub struct RemoveSplitSubaccountBody<'a> {
    /// This is the sub account code
    pub subaccount: &'a str,
}

impl TransactionSplit {
    /// Create a split payment on your integration
    pub fn create_split(&self, body: CreateSplitPaymentBody) -> Result<Response, String> {
        let res = make_request(
            &self.bearer_auth,
            &SPLIT_PAYMENT_URL,
            Some(body),
            REQUEST::POST,
        );
        return res;
    }

    /// List/search for the transaction splits available on your integration.
    pub fn list_or_search_splits(
        &self,
        params: Option<ListOrSearchSplitsParams>,
    ) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, SPLIT_PAYMENT_URL, params);
        return res;
    }

    /// Get details of a split on your integration.
    pub fn fetch_split(&self, id: &str) -> Result<Response, String> {
        let url = format!("{}/{}", SPLIT_PAYMENT_URL, id);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Update a transaction split details on your integration
    pub fn update_split(&self, id: &str, body: UpdateSplitBody) -> Result<Response, String> {
        let url = format!("{}/{}", SPLIT_PAYMENT_URL, id);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::PUT);
        return res;
    }

    /// Add a Subaccount to a Transaction Split, or update the share of an existing Subaccount in a Transaction Split
    pub fn add_or_update_split_subaccount(
        &self,
        id: &str,
        body: AddOrUpdateSplitSubaccountBody,
    ) -> Result<Response, String> {
        let url = format!("{}/{}/subaccount/add", SPLIT_PAYMENT_URL, id);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// Remove a subaccount from a transaction split
    pub fn remove_split_subaccount(
        &self,
        id: &str,
        body: RemoveSplitSubaccountBody,
    ) -> Result<Response, String> {
        let url = format!("{}/{}/subaccount/remove", SPLIT_PAYMENT_URL, id);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }
}
