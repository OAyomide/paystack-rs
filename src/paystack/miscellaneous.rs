use reqwest::blocking::Response;
use serde::Serialize;

use crate::{prelude::Currency, utils::make_get_request};

/// The Miscellaneous API are supporting APIs that can be used to provide more details to other APIs
#[derive(Debug, Default)]
pub struct Miscellaneous {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Gateway {
    Emandate,
    DigitalBankMandate,
}
const LIST_BANKS_URL: &str = "https://api.paystack.co/bank";
const LIST_COUNTRIES_URL: &str = "https://api.paystack.co/country";
const LIST_STATES_URL: &str = "https://api.paystack.co/address_verification/states";
#[derive(Debug, Serialize)]
pub struct ListBanksParams<'a> {
    /// The country from which to obtain the list of supported banks. e.g `country=ghana` or `country=nigeria`
    pub country: &'a str,
    /// Flag to enable cursor pagination on the endpoint
    pub use_cursor: bool,
    #[serde(rename = "perPage")]
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    pub per_page: i64,
    /// A cursor that indicates your place in the list. It can be used to fetch the next page of the list
    pub next: Option<&'a str>,
    /// A cursor that indicates your place in the list. It should be used to fetch the previous page of the list after an intial next request
    pub previous: Option<&'a str>,
    /// The gateway type of the bank. It can be one of these: [emandate, digitalbankmandate]
    pub gateway: Option<Gateway>,
    /// Type of financial channel. For Ghanaian channels, please use either **mobile_money** for mobile money channels OR **ghipps** for bank channels
    #[serde(rename = "type")]
    pub ttype: &'a str,
    /// Any of `NGN`, `USD`, `GHS` or `ZAR`
    pub currency: Option<Currency>,
}

#[derive(Debug, Serialize)]
pub struct ListProvidersParams {
    /// A flag to filter for available providers
    pub pay_with_bank_transfer: bool,
}

#[derive(Debug, Serialize)]
pub struct ListStatesParams {
    /// The country code of the states to list. It is gotten after the charge request.
    pub country: i64,
}
impl Miscellaneous {
    /// Get a list of all supported banks and their properties
    pub fn list_banks(&self, params: ListBanksParams) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, LIST_BANKS_URL, Some(params));
        return res;
    }

    // TODO: link with dedicated nuban
    /// Get a list of all providers for [][Dedicated NUBAN]
    pub fn list_providers(&self, params: ListProvidersParams) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, LIST_BANKS_URL, Some(params));
        return res;
    }

    /// Gets a list of Countries that Paystack currently supports
    pub fn list_or_search_countries(&self) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, LIST_COUNTRIES_URL, None::<String>);
        return res;
    }

    /// Get a list of states for a country for address verification.
    pub fn list_states(&self, params: ListStatesParams) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, LIST_STATES_URL, Some(params));
        return res;
    }
}
