use crate::{
    prelude::Status,
    utils::{make_get_request, make_request, REQUEST},
};
use chrono::{DateTime, Local};
use reqwest::blocking::Response;
use serde::Serialize;

/// The Bulk Charges API allows you create and manage multiple recurring payments from your customers
#[derive(Debug, Default)]
pub struct BulkCharges {
    pub(crate) bearer_auth: String,
}

// #[derive(Debug, Serialize)]
// pub struct InitiateBulkChargesBody {
//  pub
// }

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BulkChargesStatus {
    FAILED,
    SUCCESS,
    PENDING,
}

#[derive(Debug, Serialize)]
pub struct ListBulkChargesParams {
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
pub struct FetchChargesInABatchParams {
    /// Either one of these values: pending, success or failed
    pub status: BulkChargesStatus,
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
const BULK_CHARGES_URL: &str = "https://api.paystack.co/bulkcharge";
impl BulkCharges {
    // FIXME: the docs dont say what it is here, hence I wont be implementing this method until the docs are clear
    // pub fn initiate_bulk_charges(&self, body: InitiateBulkChargesBody) -> Result<Response, String> {
    //     let res = make_request(
    //         &self.bearer_auth,
    //         BULK_CHARGES_URL,
    //         Some(body),
    //         REQUEST::POST,
    //     );
    //     return res;
    // }

    /// This lists all bulk charge batches created by the integration. Statuses can be active, paused, or complete.
    pub fn list_bulk_charges(
        &self,
        params: Option<ListBulkChargesParams>,
    ) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, BULK_CHARGES_URL, params);
        return res;
    }

    /// This endpoint retrieves a specific batch code.
    /// It also returns useful information on its progress by way of the `total_charges` and `pending_charges` attributes.
    /// - id_or_code:
    /// An ID or code for the charge whose batches you want to retrieve.
    pub fn fetch_bulk_charge_batch(&self, id_or_code: &str) -> Result<Response, String> {
        let url = format!("{}/{}", BULK_CHARGES_URL, id_or_code);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }
    /// This endpoint retrieves the charges associated with a specified batch code. Pagination parameters are available.
    /// You can also filter by status. Charge statuses can be pending, success or failed.
    /// - id_or_code:
    /// An ID or code for the charge whose batches you want to retrieve.
    pub fn fetch_charges_in_a_batch(
        &self,
        id_or_code: &str,
        params: FetchChargesInABatchParams,
    ) -> Result<Response, String> {
        let url = format!("{}/{}", BULK_CHARGES_URL, id_or_code);
        let res = make_get_request(&self.bearer_auth, &url, Some(params));
        return res;
    }

    /// Use this endpoint to pause processing a batch
    /// - batch_code:
    /// The batch code for the bulk charge you want to pause
    pub fn pause_bulk_charge_batch(&self, batch_code: &str) -> Result<Response, String> {
        let url = format!("{}/pause/{}", BULK_CHARGES_URL, batch_code);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Use this endpoint to pause processing a batch
    /// - batch_code:
    /// The batch code for the bulk charge you want to pause
    pub fn resume_bulk_charge_batch(&self, batch_code: &str) -> Result<Response, String> {
        let url = format!("{}/resume/{}", BULK_CHARGES_URL, batch_code);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }
}
