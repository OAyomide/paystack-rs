use crate::{
    prelude::Currency,
    utils::{make_get_request, make_post_request},
};
use chrono::{DateTime, Utc};
use reqwest::{
    blocking::{Client, Response},
    header::AUTHORIZATION,
    StatusCode,
};
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

#[derive(Debug)]
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
        let res = make_post_request(self.bearer_auth.clone(), SPLIT_PAYMENT_URL.to_owned(), body);
        return res;
    }

    /// List/search for the transaction splits available on your integration.
    pub fn list_or_search_splits(
        &self,
        params: ListOrSearchSplitsParams,
    ) -> Result<Response, String> {
        let reqwest_client = Client::new();
        let res = reqwest_client
            .get(SPLIT_PAYMENT_URL.to_owned())
            .header(AUTHORIZATION, self.bearer_auth.clone())
            .query(&[
                ("name", params.name),
                ("sort_by", params.sorted_by.unwrap()),
            ])
            .query(&[("active", params.active)])
            .query(&[
                ("perPage", params.per_page.unwrap()),
                ("page", params.page.unwrap()),
            ])
            .query(&[("from", params.from.unwrap()), ("to", params.to.unwrap())])
            .send()
            .expect("Error listing or searching split payments");
        match res.status() {
            StatusCode::OK => return Ok(res),
            StatusCode::BAD_REQUEST => return Err("Bad request. Please check the body".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR => {
                return Err("An error occured on the paystack server: please try again".to_string())
            }
            _ => return Ok(res),
        }
    }

    /// Get details of a split on your integration.
    pub fn fetch_split(&self, id: &str) -> Result<Response, String> {
        let url = format!("{}/{}", SPLIT_PAYMENT_URL.to_owned(), id);
        let res = make_get_request(self.bearer_auth.clone(), url);
        return res;
    }

    /// Update a transaction split details on your integration
    pub fn update_split(&self, id: &str, body: UpdateSplitBody) -> Result<Response, String> {
        let url = format!("{}/{}", SPLIT_PAYMENT_URL.to_owned(), id);
        let res = make_post_request(self.bearer_auth.clone(), url, body);
        return res;
    }

    /// Add a Subaccount to a Transaction Split, or update the share of an existing Subaccount in a Transaction Split
    pub fn add_or_update_split_subaccount(
        &self,
        id: &str,
        body: AddOrUpdateSplitSubaccountBody,
    ) -> Result<Response, String> {
        let url = format!("{}/{}/subaccount/add", SPLIT_PAYMENT_URL.to_owned(), id);
        let res = make_post_request(self.bearer_auth.clone(), url, body);
        return res;
    }

    /// Remove a subaccount from a transaction split
    pub fn remove_split_subaccount(
        &self,
        id: &str,
        body: RemoveSplitSubaccountBody,
    ) -> Result<Response, String> {
        let url = format!("{}/{}/subaccount/remove", SPLIT_PAYMENT_URL.to_owned(), id);
        let res = make_post_request(self.bearer_auth.clone(), url, body);
        return res;
    }
}
