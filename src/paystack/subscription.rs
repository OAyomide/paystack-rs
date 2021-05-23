use crate::utils::{make_get_request, make_request, REQUEST};
use chrono::{DateTime, Utc};
use reqwest::blocking::Response;
use serde::Serialize;

/// The Subscriptions API allows you create and manage recurring payment on your integration
#[derive(Debug, Default)]
pub struct Subscription {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct CreateSubscriptionBody {
    /// Customer's email address or customer code
    pub customer: String,
    /// Plan code
    pub plan: String,
    /// If customer has multiple authorizations, you can set the desired authorization you wish to use for this subscription here. If this is not supplied, the customer's most recent authorization would be used
    pub authorization: String,
    /// Set the date for the first debit. (ISO 8601 format) e.g. 2017-05-16T00:30:13+01:00
    pub start_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSubscriptionParams {
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    pub per_page: i128,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: i128,
    /// Filter by Customer ID
    pub customer: Option<i128>,
    /// Filter by Plan ID
    pub plan: Option<i128>,
}

#[derive(Debug, Serialize)]
pub struct EnableSubscriptionBody {
    /// Subscription code
    pub code: String,
    /// Email token
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct DisableSubscriptionBody {
    /// Subscription code
    pub code: String,
    /// Email token
    pub token: String,
}

const SUBSCRIPTION_URL: &str = "https://api.paystack.co/subscription";
/// The Subscriptions API allows you create and manage recurring payment on your integration
impl Subscription {
    /// Create a subscription on your integration
    /// 💡 Email Token We create an email token on each subscription to allow customers cancel their subscriptions from within the invoices sent to their mailboxes. Since they are not authorized, the email tokens are what we use to authenticate the requests over the API.
    pub fn create_subscription(&self, body: CreateSubscriptionBody) -> Result<Response, String> {
        let res = make_request(
            &self.bearer_auth,
            SUBSCRIPTION_URL,
            Some(body),
            REQUEST::POST,
        );
        return res;
    }

    /// List subscriptions available on your integration.
    pub fn list_subscription(
        &self,
        params: Option<ListSubscriptionParams>,
    ) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, SUBSCRIPTION_URL, params);
        return res;
    }
    /// Get details of a subscription on your integration.
    /// id_or_code: The subscription ID or code you want to fetch
    pub fn fetch_subscription(&self, id_or_code: String) -> Result<Response, String> {
        let url = format!("{}/{}", SUBSCRIPTION_URL.to_owned(), id_or_code);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }
    /// Enable a subscription on your integration
    pub fn enable_subscription(&self, body: EnableSubscriptionBody) -> Result<Response, String> {
        let url = format!("{}/enable", SUBSCRIPTION_URL.to_owned());
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// Disable a subscription on your integration
    pub fn disable_subscription(&self, body: DisableSubscriptionBody) -> Result<Response, String> {
        let url = format!("{}/disable", SUBSCRIPTION_URL.to_owned());
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }
}
