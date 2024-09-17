use crate::utils::*;
use chrono::{prelude::DateTime, Utc};
use reqwest::blocking::Response;
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;

#[derive(Default)]
/// The Transactions API allows you create and manage payments on your integration
pub struct Transaction {
    pub(crate) bearer_auth: String,
}

#[derive(Debug, Serialize)]
pub enum Currency {
    NGN,
    GHS,
    USD,
    ZAR,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    FAILED,
    SUCCESS,
    ABANDONED,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChargesBearer {
    Account,
    Subaccount,
}
impl Default for ChargesBearer {
    fn default() -> Self {
        ChargesBearer::Account
    }
}
impl Default for Currency {
    fn default() -> Self {
        Currency::NGN
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Channels {
    Card,
    Bank,
    Ussd,
    Qr,
    MobileMoney,
    BankTransfer,
}

const CHARGE_AUTHORIZATION_URL: &str = "https://api.paystack.co/transaction/charge_authorization";
const INITIALIZE_TRANSACTION_URL: &str = "https://api.paystack.co/transaction/initialize";
const PAYSTACK_BASE_URL: &str = "https://api.paystack.co";
const TRANSACTION_URL: &str = "https://api.paystack.co/transaction";
/// struct passed to initiatialize a transaction.
#[derive(Debug, Default, Serialize)]
pub struct InitializeTransactionBody {
    /// Customer's email address
    pub email: String,
    /// Amount should be in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR
    pub amount: i128, // tbh, not sure what integer type i should use here. but pretty sure you cannot go wrong with i128
    /// The transaction currency (NGN, GHS, ZAR or USD). Defaults to your integration currency.
    pub currency: Option<Currency>,
    /// Unique transaction reference. Only -, ., = and alphanumeric characters allowed.
    pub reference: Option<String>,
    /// Fully qualified url, e.g. https://example.com/ . Use this to override the callback url provided on the dashboard for this transaction
    pub callback_url: Option<String>,
    /// If transaction is to create a subscription to a predefined plan, provide plan code here. This would invalidate the value provided in amount
    pub plan: Option<String>,
    /// Number of times to charge customer during subscription to plan
    pub invoice_limit: Option<i64>,
    /// Stringified JSON object of custom data. Kindly check the Metadata page for more information
    pub metadata: Option<String>,
    /// An array of payment channels to control what channels you want to make available to the user to make a payment with. Available channels include: ['card', 'bank', 'ussd', 'qr', 'mobile_money', 'bank_transfer']
    pub channels: Option<Vec<Channels>>,
    /// The split code of the transaction split. e.g. SPL_98WF13Eb3w
    pub split_code: Option<String>,
    /// The code for the subaccount that owns the payment. e.g. ACCT_8f4s1eq7ml6rlzj
    pub subaccount: Option<String>,
    /// A flat fee to charge the subaccount for this transaction (). This overrides the split percentage set when the subaccount was created. Ideally, you will need to use this if you are splitting in flat rates (since subaccount creation only allows for percentage split). e.g. 7000 for a 70 naira flat fee.
    pub transaction_charge: Option<i128>,
    /// Who bears Paystack charges? account or subaccount (defaults to account).
    pub bearer: Option<ChargesBearer>,
}

/// struct ListTransactionsQuery
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsParams {
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    pub per_page: Option<i64>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i64>,
    /// Specify an ID for the customer whose transactions you want to retrieve
    pub customer: Option<i64>,
    /// Filter transactions by status ('failed', 'success', 'abandoned')
    pub status: Option<Status>,
    /// A timestamp from which to start listing transaction e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: Option<DateTime<Utc>>,
    /// A timestamp at which to stop listing transaction e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: Option<DateTime<Utc>>,
    /// Filter transactions by amount. Specify the amount (in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR)
    pub amount: Option<i128>,
}

#[derive(Serialize, Default, Debug)]
pub struct ChargeAuthorizationBody {
    /// Amount should be in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR
    pub amount: String,
    /// Customer's email address
    pub email: String,
    /// Valid authorization code to charge
    pub authorization_code: String,
    /// Unique transaction reference. Only -, ., = and alphanumeric characters allowed.
    pub reference: Option<String>,
    /// Currency in which amount should be charged. Allowed values are: NGN, GHS, ZAR or USD
    pub currency: Option<Currency>,
    /// Stringified JSON object. Add a custom_fields attribute which has an array of objects if you would like the fields to be added to your transaction when displayed on the dashboard. Sample: {"custom_fields":[{"display_name":"Cart ID","variable_name": "cart_id","value": "8393"}]}
    pub metadata: Option<Value>,
    /// Send us 'card' or 'bank' or 'card','bank' as an array to specify what options to show the user paying
    pub channels: Option<Vec<String>>,
    /// The code for the subaccount that owns the payment. e.g. ACCT_8f4s1eq7ml6rlzj
    pub subaccount: Option<String>,
    /// A flat fee to charge the subaccount for this transaction (in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR). This overrides the split percentage set when the subaccount was created. Ideally, you will need to use this if you are splitting in flat rates (since subaccount creation only allows for percentage split). e.g. 7000 for a 70 naira
    pub transaction_charge: Option<i128>,
    /// Who bears Paystack charges? account or subaccount (defaults to account).
    pub bearer: Option<ChargesBearer>,
    /// If you are making a scheduled charge call, it is a good idea to queue them so the processing system does not get overloaded causing transaction processing errors. Send queue:true to take advantage of our queued charging.
    pub queue: Option<bool>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsTotal {
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    pub per_page: Option<i64>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i64>,
    /// Specify an ID for the customer whose transactions you want to retrieve
    pub customer: Option<i64>,
    /// Filter transactions by status ('failed', 'success', 'abandoned')
    pub status: Option<Status>,
    /// A timestamp from which to start listing transaction e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: Option<DateTime<Utc>>,
    /// A timestamp at which to stop listing transaction e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Serialize)]
pub struct CheckAuthorizationBody {
    /// Amount should be in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR
    pub amount: String,
    /// Customer's email address
    pub email: String,
    /// Valid authorization code to charge
    pub authorization_code: String,
    /// Currency in which amount should be charged. Allowed values are: NGN, GHS, ZAR or USD
    pub currency: Option<Currency>,
}

#[derive(Debug, Default, Serialize)]
pub struct PartialDebitBody {
    /// Amount should be in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR
    pub amount: String,
    /// Customer's email address (attached to the authorization code)
    pub email: String,
    /// Authorization Code
    pub authorization_code: String,
    /// Specify the currency you want to debit. Allowed values are NGN, GHS, ZAR or USD.
    pub currency: Currency,
    /// Unique transaction reference. Only -, ., = and alphanumeric characters allowed.
    pub reference: Option<String>,
    /// Minimum amount to charge
    pub at_least: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExportTransactionsBody {
    // FIXME: at this moment, I dont know if this will work but rust-analyzer isnt throwing an error so... 🤷🏾‍♂️
    #[serde(rename = "perPage")]
    /// Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    pub per_page: Option<i64>,
    /// Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    pub page: Option<i64>,
    /// A timestamp from which to start listing transaction e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub from: Option<DateTime<Utc>>,
    /// A timestamp at which to stop listing transaction e.g. 2016-09-24T00:00:05.000Z, 2016-09-21
    pub to: Option<DateTime<Utc>>,
    /// Specify an ID for the customer whose transactions you want to retrieve
    pub customer: Option<i64>,
    /// Filter transactions by status ('failed', 'success', 'abandoned')
    pub status: Option<Status>,
    /// Specify the transaction currency to export. Allowed values are: in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR
    pub currency: Option<Currency>,
    /// Filter transactions by amount. Specify the amount, in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR
    pub amount: Option<String>,
    /// Set to true to export only settled transactions. false for pending transactions. Leave undefined to export all transactions
    pub settled: Option<bool>,
    /// An ID for the settlement whose transactions we should export
    pub settlement: Option<i64>,
    /// Specify a payment page's id to export only transactions conducted on said page
    pub payment_page: Option<i64>,
}

impl Transaction {
    /// Initialize a transaction from your backend
    /// ```rust
    /// # use std::env;
    /// # use paystack_rs::prelude::Paystack;
    /// use paystack_rs::prelude::InitializeTransactionBody;
    ///
    /// # let key = env::var("PAYSTACK_SECRET_KEY").unwrap();
    /// let paystack = Paystack::new(key);
    /// let body = InitializeTransactionBody{
    ///     email: "randomemail@gmail.com".to_string(),
    ///     amount: 10000,
    ///     ..Default::default()
    /// };
    /// paystack.transaction.initialize_transaction(body);
    /// ```
    pub fn initialize_transaction(
        &self,
        body: InitializeTransactionBody,
    ) -> Result<Response, String> {
        let res = make_request(
            &self.bearer_auth,
            INITIALIZE_TRANSACTION_URL,
            Some(body),
            REQUEST::POST,
        );
        return res;
    }

    /// verify a transaction. it takes an argument reference which is the reference_id of a transaction you want to verify
    /// ```rust
    /// # use std::env;
    /// # use paystack_rs::prelude::Paystack;
    ///
    /// # let key = env::var("PAYSTACK_SECRET_KEY").unwrap();
    /// let paystack = Paystack::new(key);
    /// paystack.transaction.verify_transaction("DG4uishudoq90LD".to_string());
    /// ```
    pub fn verify_transaction(&self, reference: String) -> Result<Response, String> {
        let full_url = format!(
            "{}/transaction/verify/:{}",
            PAYSTACK_BASE_URL,
            reference.to_string()
        );
        let result = make_get_request(&self.bearer_auth, &full_url, None::<String>);
        return result;
    }

    /// list_transactions lists all the transactions available
    /// ```rust
    /// # use std::env;
    /// # use paystack_rs::prelude::Paystack;
    /// use paystack_rs::prelude::ListTransactionsParams;
    ///
    /// # let key = env::var("PAYSTACK_SECRET_KEY").unwrap();
    /// let paystack = Paystack::new(key);
    /// /// Retrieve 50 transactions per page
    /// let body = ListTransactionsParams{
    ///     per_page: Some(50),
    ///     ..Default::default()
    /// };
    /// paystack.transaction.list_transactions(body);
    pub fn list_transactions(&self, body: ListTransactionsParams) -> Result<Response, String> {
        let res = make_get_request(&self.bearer_auth, TRANSACTION_URL, Some(body));
        return res;
    }

    /// ```rust
    /// # use std::env;
    /// # use paystack_rs::prelude::Paystack;
    ///
    /// # let key = env::var("PAYSTACK_SECRET_KEY").unwrap();
    /// let paystack = Paystack::new(key);
    /// paystack.transaction.fetch_transaction(123412);
    /// ```
    pub fn fetch_transaction(&self, transaction_id: i64) -> Result<Response, String> {
        let url = format!("{}/{}", TRANSACTION_URL, transaction_id);
        let res = make_get_request(&self.bearer_auth, &url, None::<String>);
        return res;
    }

    /// All authorizations marked as reusable can be charged with this endpoint whenever you need to receive payments.
    /// ```rust
    /// # use std::env;
    /// # use paystack_rs::prelude::Paystack;
    /// use paystack_rs::prelude::ChargeAuthorizationBody;
    ///
    /// # let key = env::var("PAYSTACK_SECRET_KEY").unwrap();
    /// let paystack = Paystack::new(key);
    /// let body = ChargeAuthorizationBody{
    ///     amount: "5000".to_string(),
    ///     email: "randomemail@gmail.com".to_string(),
    ///     authorization_code: "2aeserqwdEAW".to_string(),
    ///     ..Default::default()
    /// };
    /// paystack.transaction.charge_authorization(body);
    pub fn charge_authorization(
        &self,
        params: ChargeAuthorizationBody,
    ) -> Result<Response, String> {
        let res = make_request(
            &self.bearer_auth,
            CHARGE_AUTHORIZATION_URL,
            Some(params),
            REQUEST::POST,
        );
        return res;
    }
    /// All mastercard and visa authorizations can be checked with this endpoint to know if they have funds for the payment you seek.
    /// This endpoint should be used when you do not know the exact amount to charge a card when rendering a service. It should be used to check if a card has enough funds based on a maximum range value. It is well suited for:
    ///
    ///  - [x] Ride hailing services
    ///  - [x] Logistics services.
    ///
    ///
    /// ⚠️ Warning You shouldn't use this endpoint to check a card for sufficient funds if you are going to charge the user immediately. This is because we hold funds when this endpoint is called which can lead to an insufficient funds error.
    /// ```rust
    /// # use std::env;
    /// # use paystack_rs::prelude::Paystack;
    /// use paystack_rs::prelude::CheckAuthorizationBody;
    ///
    /// # let key = env::var("PAYSTACK_SECRET_KEY").unwrap();
    /// let paystack = Paystack::new(key);
    /// let body = CheckAuthorizationBody{
    ///     amount: "5000".to_string(),
    ///     email: "randomemail@gmail.com".to_string(),
    ///     authorization_code: "2aeserqwdEAW".to_string(),
    ///     ..Default::default()
    /// };
    /// paystack.transaction.check_authorization(body);
    pub fn check_authorization(&self, param: CheckAuthorizationBody) -> Result<Response, String> {
        let full_url = CHARGE_AUTHORIZATION_URL;
        let res = make_request(&self.bearer_auth, full_url, Some(param), REQUEST::POST);
        return res;
    }

    /// ```rust
    /// # use std::env;
    /// # use paystack_rs::prelude::Paystack;
    ///
    /// # let key = env::var("PAYSTACK_SECRET_KEY").unwrap();
    /// let paystack = Paystack::new(key);
    /// paystack.transaction.view_transaction_timeline("DG4uishudoq90LD".to_string());
    pub fn view_transaction_timeline(&self, id: String) -> Result<Response, String> {
        let full_url = format!("{}/timeline/{}", TRANSACTION_URL, id).to_string();
        let res = make_get_request(&self.bearer_auth, &full_url, None::<String>);
        return res;
    }

    /// Total amount received on your account
    /// ```rust
    /// # use std::env;
    /// # use paystack_rs::prelude::Paystack;
    ///
    /// # let key = env::var("PAYSTACK_SECRET_KEY").unwrap();
    /// let paystack = Paystack::new(key);
    /// /// Retrieve total transactions
    /// paystack.transaction.transactions_total(None);
    pub fn transactions_total(
        &self,
        params: Option<TransactionsTotal>,
    ) -> Result<Response, String> {
        let full_url = format!("{}/totals", TRANSACTION_URL);
        let res = make_get_request(&self.bearer_auth, &full_url, params);
        return res;
    }

    /// Export transactions carried out on your integration.
    /// ```rust
    /// # use std::env;
    /// # use paystack_rs::prelude::Paystack;
    ///
    /// # let key = env::var("PAYSTACK_SECRET_KEY").unwrap();
    /// let paystack = Paystack::new(key);
    /// /// Retrieve total transactions
    /// paystack.transaction.export_transactions(None);
    pub fn export_transactions(
        &self,
        params: Option<ExportTransactionsBody>,
    ) -> Result<Response, String> {
        let full_url = format!("{}/export", TRANSACTION_URL);
        let res = make_get_request(&self.bearer_auth, &full_url, params);
        return res;
    }

    /// Retrieve part of a payment from a customer
    /// ```rust
    /// # use std::env;
    /// # use paystack_rs::prelude::Paystack;
    /// use paystack_rs::prelude::PartialDebitBody;
    ///
    /// # let key = env::var("PAYSTACK_SECRET_KEY").unwrap();
    /// let paystack = Paystack::new(key);
    /// let body = PartialDebitBody{
    ///     amount: "5000".to_string(),
    ///     email: "randomemail@gmail.com".to_string(),
    ///     authorization_code: "2aeserqwdEAW".to_string(),
    ///     ..Default::default()
    /// };
    /// paystack.transaction.partial_debit(body);
    pub fn partial_debit(&self, body: PartialDebitBody) -> Result<Response, String> {
        let full_url = format!("{}/partial_debit", TRANSACTION_URL);
        let res = make_request(&self.bearer_auth, &full_url, Some(body), REQUEST::POST);
        return res;
    }
}
