use chrono::{DateTime, Utc};
use reqwest::blocking::Response;
use serde::Serialize;

use crate::{
    prelude::Currency,
    utils::{make_get_request, make_request, REQUEST},
};

const PRODUCT_URL: &str = "https://api.paystack.co/product";
#[derive(Debug, Default)]
/// The Products API allows you create and manage inventories on your integration
pub struct Products {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct CreateProductBody {
    /// Name of product
    pub name: String,
    /// A description for this product
    pub description: String,
    /// Price should be in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR
    pub price: i128,
    /// Currency in which price is set. Allowed values are: NGN, GHS, ZAR or USD
    pub currency: Currency,
    /// Set to true if the product has limited stock. Leave as false if the product has unlimited stock
    pub limited: Option<bool>,
    /// Number of products in stock. Use if limited is true
    pub quantity: Option<i128>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListProductsParams {
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    pub per_page: Option<i128>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i128>,
    /// A timestamp from which to start listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: Option<DateTime<Utc>>,
    /// A timestamp at which to stop listing product e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct UpdateProductsBody {
    /// Name of product
    pub name: String,
    /// A description for this product
    pub description: String,
    /// Price should be in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR
    pub price: i128,
    /// Currency in which price is set. Allowed values are: NGN, GHS, ZAR or USD
    pub currency: Currency,
    /// Set to true if the product has limited stock. Leave as false if the product has unlimited stock
    pub limited: Option<bool>,
    /// Number of products in stock. Use if limited is true
    pub quantity: Option<i128>,
}

impl Products {
    /// Create a product on your integration
    pub fn create_products(&self, body: CreateProductBody) -> Result<Response, String> {
        let res = make_request(&self.bearer_auth, PRODUCT_URL, Some(body), REQUEST::POST);
        return res;
    }

    /// List products available on your integration.
    pub fn list_products(&self, params: Option<ListProductsParams>) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, PRODUCT_URL, params);
        return res;
    }

    /// Get details of a product on your integration.
    /// id: The product ID you want to fetch
    pub fn fetch_products(&self, id: String) -> Result<Response, String> {
        let url = format!("{}/{}", PRODUCT_URL.to_owned(), id);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Update a product details on your integration.
    pub fn update_products(
        &self,
        id: String,
        body: UpdateProductsBody,
    ) -> Result<Response, String> {
        let url = format!("{}/{}", PRODUCT_URL.to_owned(), id);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::PUT);
        return res;
    }
}
