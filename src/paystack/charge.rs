use chrono::{DateTime, Local};
use reqwest::blocking::Response;
use serde::Serialize;
use serde_json::Value as JSON;

use crate::utils::{make_get_request, make_request, REQUEST};

#[derive(Debug, Default)]
pub struct Charge {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub struct CreateChargeBody<'a> {
    /// Customer's email address
    pub email: &'a str,
    /// Amount should be in kobo if currency is `NGN`, pesewas, if currency is `GHS`, and cents, if currency is `ZAR`
    pub amount: &'a str,
    /// Bank account to charge (don't send if charging an authorization code)
    pub bank: Option<JSON>,
    /// An authorization code to charge (don't send if charging a bank account)
    pub authorization_code: Option<&'a str>,
    /// 4-digit PIN (send with a non-reusable authorization code)
    pub pin: Option<&'a str>,
    /// A JSON object
    pub metadata: Option<JSON>,
    /// Unique transaction reference. Only -, .`, = and alphanumeric characters allowed.
    pub reference: Option<&'a str>,
    /// USSD type to charge (don't send if charging an authorization code, bank or card)
    pub ussd: Option<JSON>,
    /// Mobile details (don't send if charging an authorization code, bank or card)
    pub mobile_money: Option<JSON>,
    /// This is the unique identifier of the device a user uses in making payment.
    /// Only -, .`, = and alphanumeric characters allowed.
    pub device_id: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct SubmitPinBody<'a> {
    /// PIN submitted by user
    pub pin: &'a str,
    /// Reference for transaction that requested pin
    pub reference: &'a str,
}

#[derive(Debug, Serialize)]
pub struct SubmitOTPBody<'a> {
    /// OTP submitted by user
    pub otp: &'a str,
    /// Reference for ongoing transaction
    pub reference: &'a str,
}

#[derive(Debug, Serialize)]
pub struct SubmitPhoneBody<'a> {
    /// Phone submitted by user
    pub phone: &'a str,
    /// Reference for ongoing transaction
    pub reference: &'a str,
}

#[derive(Debug, Serialize)]
pub struct SubmitBirthdayBody<'a> {
    /// Birthday submitted by user e.g. 2016-09-21
    pub birthday: DateTime<Local>,
    /// Reference for ongoing transaction
    pub reference: &'a str,
}

#[derive(Debug, Serialize)]
pub struct SubmitAddressBody<'a> {
    /// Address submitted by user
    pub address: &'a str,
    /// Reference for ongoing transaction
    pub reference: &'a str,
    /// City submitted by user
    pub city: &'a str,
    /// State submitted by user
    pub state: &'a str,
    /// Zipcode submitted by user
    pub zipcode: &'a str,
}

const CHARGE_URL: &str = "https://api.paystack.co/charge";
impl Charge {
    // TODO: link payment channel here
    /// Initiate a payment by integrating the [][payment channel] of your choice.
    pub fn create_charge(&self, body: CreateChargeBody) -> Result<Response, String> {
        let res = make_request(&self.bearer_auth, CHARGE_URL, Some(body), REQUEST::POST);
        return res;
    }

    /// Submit PIN to continue a charge
    pub fn submit_pin(&self, body: SubmitPinBody) -> Result<Response, String> {
        let url = format!("{}/submit_pin", CHARGE_URL);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// Submit OTP to complete a charge
    pub fn submit_otp(&self, body: SubmitOTPBody) -> Result<Response, String> {
        let url = format!("{}/submit_otp", CHARGE_URL);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// Submit phone when requested
    pub fn submit_phone(&self, body: SubmitPhoneBody) -> Result<Response, String> {
        let url = format!("{}/submit_phone", CHARGE_URL);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// Submit birthday when requested
    pub fn submit_birthday(&self, body: SubmitBirthdayBody) -> Result<Response, String> {
        let url = format!("{}/submit_birthday", CHARGE_URL);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// Submit address to continue a charge
    pub fn submit_address(&self, body: SubmitAddressBody) -> Result<Response, String> {
        let url = format!("{}/submit_address", CHARGE_URL);
        let res = make_request(&self.bearer_auth, &url, Some(body), REQUEST::POST);
        return res;
    }

    /// When you get "pending" as a charge status or if there was an exception when calling any of the /charge endpoints,
    /// wait 10 seconds or more, then make a check to see if its status has changed.
    /// Don't call too early as you may get a lot more pending than you should.
    pub fn check_pending_charge(&self, reference: &str) -> Result<Response, String> {
        let url = format!("{}/{}", CHARGE_URL, reference);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }
}
