use chrono::{DateTime, Local};
use reqwest::blocking::Response;
use serde::Serialize;
use serde_json::Value as JSON;

use crate::utils::{make_get_request, make_request, REQUEST};

#[derive(Debug, Default)]
/// The Payment Pages API provides a quick and secure way to collect payment for products.
pub struct PaymentPages {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct CreatePaymentPagesBody<'a> {
    /// Name of page
    pub name: &'a str,
    /// A description for this page
    pub description: Option<&'a str>,
    /// Amount should be in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR
    pub amount: Option<i128>,
    /// URL slug you would like to be associated with this page. Page will be accessible at https://paystack.com/pay/[slug]
    pub slug: Option<String>,
    /// Extra data to configure the payment page including subaccount, logo image, transaction charge
    pub metadata: Option<JSON>,
    /// If you would like Paystack to redirect someplace upon successful payment, specify the URL here.
    pub redirect_url: Option<&'a str>,
    /// If you would like to accept custom fields, specify them here.
    pub custom_fields: Option<Vec<JSON>>,
}

#[derive(Debug, Serialize)]
pub struct ListPagesParams {
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    #[serde(rename = "perPage")]
    pub per_page: Option<i128>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i128>,
    /// A timestamp from which to start listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: Option<DateTime<Local>>,
    /// A timestamp at which to stop listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize)]
pub struct UpdatePageBody<'a> {
    /// Name of page
    pub name: &'a str,
    /// A description for this page
    pub description: Option<&'a str>,
    /// Default amount you want to accept using this page. If none is set, customer is free to provide any amount of their choice. The latter scenario is useful for accepting donations
    pub amount: Option<i128>,
    /// Set to false to deactivate page url
    pub active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AddProductsBody {
    /// Ids of all the products
    pub product: Vec<i128>,
}

const PAYMENT_PAGES_URL: &str = "https://api.paystack.co/page";
impl PaymentPages {
    /// Create a payment page on your integration
    pub fn create_pages(&self, body: CreatePaymentPagesBody) -> Result<Response, String> {
        let res = make_request(
            &self.bearer_auth,
            PAYMENT_PAGES_URL,
            Some(body),
            REQUEST::POST,
        );
        return res;
    }
    /// List payment pages available on your integration.
    pub fn list_pages(&self, params: Option<ListPagesParams>) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, PAYMENT_PAGES_URL, params);
        return res;
    }

    /// Get details of a payment page on your integration.
    /// id_or_slug: The page ID or slug you want to fetch.
    pub fn fetch_page(&self, id_or_slug: &str) -> Result<Response, String> {
        let url = format!("{}/{}", PAYMENT_PAGES_URL, id_or_slug);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Update a payment page details on your integration
    pub fn update_page(&self, id_or_slug: &str, body: UpdatePageBody) -> Result<Response, String> {
        let url = format!("{}/{}", PAYMENT_PAGES_URL, id_or_slug);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// Check the availability of a slug for a payment page.
    /// slug: URL slug to be confirmed
    pub fn check_slug_availability(&self, slug: &str) -> Result<Response, String> {
        let url = format!("{}/check_slug_availability/{}", PAYMENT_PAGES_URL, slug);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Add products to a payment page
    pub fn add_products(&self, id: i128, body: AddProductsBody) -> Result<Response, String> {
        let url = format!("{}/{}/product", PAYMENT_PAGES_URL, id);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }
}
