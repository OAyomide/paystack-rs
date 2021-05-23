use reqwest::blocking::Response;
use serde::Serialize;

use crate::utils::{make_get_request, make_request, REQUEST};

const TRANSFERS_CONTROL_URL: &str = "https://api.paystack.co/balance";

/// The Transfers Control API allows you manage settings of your transfers
#[derive(Debug, Default)]
pub struct TransfersControl {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct ResendTransfersOTPBody<'a> {
    /// Transfer code
    pub transfer_code: &'a str,
    /// Either `resend_otp` or `transfer`
    pub reason: &'a str,
}
#[derive(Debug, Serialize)]
pub struct FinalizeDisableTransferOTPBody<'a> {
    /// OTP sent to business phone to verify disabling OTP requirement
    pub otp: &'a str,
}
impl TransfersControl {
    /// Fetch the available balance on your integration
    pub fn check_balance(&self) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, TRANSFERS_CONTROL_URL, None::<String>);
        return res;
    }

    /// Fetch all pay-ins and pay-outs that occured on your integration
    pub fn fetch_balance_ledger(&self) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, TRANSFERS_CONTROL_URL, None::<String>);
        return res;
    }

    /// Generates a new OTP and sends to customer in the event they are having trouble receiving one.
    /// - 💡  Feature Availability
    /// This feature is only available to businesses in Nigeria and Ghana.
    pub fn resend_transfers_otp(&self, body: ResendTransfersOTPBody) -> Result<Response, String> {
        let url = format!("{}/resend_otp", TRANSFERS_CONTROL_URL);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// This is used in the event that you want to be able to complete transfers programmatically without use of OTPs.
    /// No arguments required. You will get an OTP to complete the request.
    /// - 💡  Feature Availability
    /// This feature is only available to businesses in Nigeria and Ghana.
    pub fn disable_transfers_otp(&self) -> Result<Response, String> {
        let url = format!("{}/disable_otp", TRANSFERS_CONTROL_URL);
        let res = make_request(&self.bearer_auth, &url, None::<String>, REQUEST::POST);
        return res;
    }

    /// Finalize the request to disable OTP on your transfers.
    /// - 💡  Feature Availability
    /// This feature is only available to businesses in Nigeria and Ghana.
    pub fn finalize_disable_transfers_otp(
        &self,
        body: FinalizeDisableTransferOTPBody,
    ) -> Result<Response, String> {
        let url = format!("{}/disable_otp_finalize", TRANSFERS_CONTROL_URL);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// In the event that a customer wants to stop being able to complete transfers programmatically, this endpoint helps turn OTP requirement back on.
    /// No arguments required.
    /// - 💡  Feature Availability
    /// This feature is only available to businesses in Nigeria and Ghana.
    pub fn enable_transfers_otp(&self) -> Result<Response, String> {
        let url = format!("{}/enable_otp", TRANSFERS_CONTROL_URL);
        let res = make_request(&self.bearer_auth, &url, None::<String>, REQUEST::POST);
        return res;
    }
}
