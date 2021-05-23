use reqwest::blocking::Response;
use serde::Serialize;

use crate::utils::{make_get_request, make_request, REQUEST};

/// The Control Panel API allows you manage some settings on your integration
#[derive(Debug, Default)]
pub struct ControlPanel {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct UpdatePaymentSessionTimeoutBody {
    /// Time before stopping session (in seconds). Set to 0 to cancel session timeouts
    pub timeout: i64,
}

const CONTROL_PANEL_URL: &str = "https://api.paystack.co/integration";
impl ControlPanel {
    /// Fetch the payment session timeout on your integration
    pub fn fetch_payment_session_timeout(&self) -> Result<Response, String> {
        let url = format!("{}/payment_session_timeout", CONTROL_PANEL_URL);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// Update the payment session timeout on your integration
    pub fn update_payment_session_timeout(
        &self,
        body: UpdatePaymentSessionTimeoutBody,
    ) -> Result<Response, String> {
        let url = format!("{}/payment_session_timeout", CONTROL_PANEL_URL);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::PUT);
        return res;
    }
}
