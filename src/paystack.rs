pub mod bulk_charges;
pub mod charge;
pub mod control_panel;
pub mod customers;
pub mod dedicated_nuban;
pub mod disputes;
pub mod invoices;
pub mod miscellaneous;
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
pub mod verification;

/// Initialize a struct with bearer auth key
#[macro_export]
macro_rules! auth_init {
    ($Foo: ident, $auth: expr) => {
        $Foo {
            bearer_auth: $auth.to_string(),
            ..Default::default()
        }
    };
}

use bulk_charges::BulkCharges;
use charge::Charge;
use control_panel::ControlPanel;
use dedicated_nuban::DedicatedNuban;
use disputes::Disputes;
use invoices::Invoices;
use miscellaneous::Miscellaneous;
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
use verification::Verification;

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
    pub control_panel: ControlPanel,
    pub charge: Charge,
    pub disputes: Disputes,
    pub verification: Verification,
    pub miscellaneous: Miscellaneous,
}

impl Paystack {
    pub fn new(key: String) -> Paystack {
        let formatted_bearer = format!("Bearer {}", key);
        Paystack {
            transaction: auth_init!(Transaction, formatted_bearer),
            transaction_split: auth_init!(TransactionSplit, formatted_bearer),
            refund: auth_init!(Refunds, formatted_bearer),
            subaccounts: auth_init!(Subaccount, formatted_bearer),
            dedicated_nuban: auth_init!(DedicatedNuban, formatted_bearer),
            plans: auth_init!(Plans, formatted_bearer),
            subscription: auth_init!(Subscription, formatted_bearer),
            products: auth_init!(Products, formatted_bearer),
            invoices: auth_init!(Invoices, formatted_bearer),
            settlements: auth_init!(Settlements, formatted_bearer),
            transfer_recipients: auth_init!(TransferRecipients, formatted_bearer),
            transfers: auth_init!(Transfers, formatted_bearer),
            transfers_control: auth_init!(TransfersControl, formatted_bearer),
            bulk_charges: auth_init!(BulkCharges, formatted_bearer),
            control_panel: auth_init!(ControlPanel, formatted_bearer),
            disputes: auth_init!(Disputes, formatted_bearer),
            verification: auth_init!(Verification, formatted_bearer),
            miscellaneous: auth_init!(Miscellaneous, formatted_bearer),
            charge: auth_init!(Charge, formatted_bearer),
            payment_pages: auth_init!(PaymentPages, formatted_bearer),
        }
    }
}
