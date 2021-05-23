pub mod bulk_charges;
pub mod customers;
pub mod dedicated_nuban;
pub mod invoices;
pub mod payment_pages;
pub mod plans;
pub mod products;
pub mod refund;
pub mod settlements;
pub mod subaccounts;
pub mod subscription;
pub mod transactions;
pub mod transactions_split;
pub mod transfer_recipients;
pub mod transfers;
pub mod transfers_control;

use bulk_charges::BulkCharges;
use dedicated_nuban::DedicatedNuban;
use invoices::Invoices;
use payment_pages::PaymentPages;
use plans::Plans;
use products::Products;
use refund::Refunds;
use settlements::Settlements;
use subaccounts::Subaccount;
use subscription::Subscription;
use transactions::Transaction;
use transactions_split::TransactionSplit;
use transfer_recipients::TransferRecipients;
use transfers::Transfers;
use transfers_control::TransfersControl;

#[derive(Default)]
pub struct Paystack {
    pub transaction: Transaction,
    pub transaction_split: TransactionSplit,
    pub refund: Refunds,
    pub subaccounts: Subaccount,
    pub dedicated_nuban: DedicatedNuban,
    pub plans: Plans,
    pub subscription: Subscription,
    pub products: Products,
    pub payment_pages: PaymentPages,
    pub invoices: Invoices,
    pub settlements: Settlements,
    pub transfer_recipients: TransferRecipients,
    pub transfers: Transfers,
    pub transfers_control: TransfersControl,
    pub bulk_charges: BulkCharges,
}

impl Paystack {
    pub fn new(key: String) -> Paystack {
        let formatted_bearer = format!("Bearer {}", key);
        Paystack {
            transaction: Transaction {
                bearer_auth: formatted_bearer.to_string(),
                ..Default::default()
            },
            transaction_split: TransactionSplit {
                bearer_auth: formatted_bearer.to_string(),
            },
            refund: Refunds {
                bearer_auth: formatted_bearer.to_string(),
            },
            subaccounts: Subaccount {
                bearer_auth: formatted_bearer.to_string(),
            },
            dedicated_nuban: DedicatedNuban {
                bearer_auth: formatted_bearer.to_string(),
            },
            plans: Plans {
                bearer_auth: formatted_bearer.to_string(),
            },
            subscription: Subscription {
                bearer_auth: formatted_bearer.to_string(),
            },
            products: Products {
                bearer_auth: formatted_bearer.to_string(),
            },
            payment_pages: PaymentPages {
                bearer_auth: formatted_bearer.to_string(),
            },
            invoices: Invoices {
                bearer_auth: formatted_bearer.to_string(),
            },
            settlements: Settlements {
                bearer_auth: formatted_bearer.to_string(),
            },
            transfer_recipients: TransferRecipients {
                bearer_auth: formatted_bearer.to_string(),
            },
            transfers: Transfers {
                bearer_auth: formatted_bearer.to_string(),
            },
            transfers_control: TransfersControl {
                bearer_auth: formatted_bearer.to_string(),
            },
            bulk_charges: BulkCharges {
                bearer_auth: formatted_bearer.to_string(),
            },
        }
    }
}
