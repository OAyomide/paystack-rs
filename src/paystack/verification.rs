use crate::utils::{make_get_request, make_request, REQUEST};
use reqwest::blocking::Response;
use serde::Serialize;

/// The Verification API allows you perform KYC processes.
///
/// *NB: due to regulations, Paystack has disabled this service.*
///  - 💡 Feature Availability
/// This feature is only available to businesses in Nigeria.
#[derive(Debug, Default)]
pub struct Verification {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyBVNBody<'a> {
    /// Bank Account Number
    pub account_number: &'a str,
    // TODO: link to list of bank here.
    /// You can get the [][list of banks] codes by calling the List Bank endpoint
    pub bank_code: i64,
    /// 11 digits Bank Verification Number
    pub bvn: &'a str,
    /// Customer's First Name
    pub first_name: Option<&'a str>,
    /// Customer's Middle Name
    pub middle_name: Option<&'a str>,
    /// Customer's Last Name
    pub last_name: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct ResolveAcctNoBody<'a> {
    /// Bank Account Number
    pub account_number: &'a str,
    // TODO: link to list of bank here.
    /// You can get the [][list of banks] codes by calling the List Bank endpoint
    pub bank_code: i64,
}
const VERIFY_BVN_MATCH_URL: &str = "https://api.paystack.co/bvn/match";
const RESOLVE_ACCT_NO_URL: &str = "https://api.paystack.co/bank/resolve";
const RESOLVE_CARD_BIN: &str = "https://api.paystack.co/decision/bin";

impl Verification {
    /// Check if an account number and BVN are linked
    pub fn verify_bvn_match(&self, body: VerifyBVNBody) -> Result<Response, String> {
        let res = make_request(
            &self.bearer_auth,
            VERIFY_BVN_MATCH_URL,
            Some(body),
            REQUEST::POST,
        );
        return res;
    }

    /// Confirm an account belongs to the right customer
    pub fn resolve_account_number(&self, params: ResolveAcctNoBody) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, RESOLVE_ACCT_NO_URL, Some(params));
        return res;
    }

    /// Get more information about a customer's card
    pub fn resolve_card_bin(&self, bin: &str) -> Result<Response, String> {
        let url = format!("{}/{}", RESOLVE_CARD_BIN, bin);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }
}
