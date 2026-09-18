use crate::dnsimple::{Client, DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

/// Represents a billing charge in DNSimple
#[derive(Debug, Deserialize, Serialize)]
pub struct Charge {
    /// When the charge was invoiced.
    pub invoiced_at: String,
    /// The aggregate amount of all line items, that need to be paid.
    pub total_amount: String,
    /// The amount that was paid via wallet.
    pub balance_amount: String,
    /// The reference number of the invoice.
    pub reference: String,
    /// The state of the charge (`collected` or `refunded`).
    pub state: String,
    /// The line items of the charge.
    pub items: Vec<ChargeItem>,
}

/// Represents a line item of a billing charge
#[derive(Debug, Deserialize, Serialize)]
pub struct ChargeItem {
    /// The description of the line item.
    pub description: String,
    /// The amount of the line item.
    pub amount: String,
    /// The ID of the product that was charged. `None` when the product type is `manual`.
    pub product_id: Option<u64>,
    /// The type of the product that was charged.
    pub product_type: String,
    /// A reference for the product that was charged, for example the domain name. `None` when the product type is `manual`.
    pub product_reference: Option<String>,
}

struct ListChargesEndpoint;

impl Endpoint for ListChargesEndpoint {
    type Output = Vec<Charge>;
}

/// The Billing Service handles the billing endpoints of the DNSimple API.
///
/// See [API Documentation: billing](https://developer.dnsimple.com/v2/billing-charges/)
pub struct Billing<'a> {
    pub client: &'a Client,
}

impl Billing<'_> {
    /// Lists the billing charges for the account.
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/billing-charges/#listCharges)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() {
    ///     let client = new_client(true, String::from("AUTH_TOKEN")).unwrap();
    ///     let response = client.billing().list_charges(1234, None).await.unwrap();
    ///     let charges = response.data.unwrap();
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `options`: The `RequestOptions`
    ///            - Filters: `start_date`, `end_date`
    ///            - Sort: `invoiced`
    pub async fn list_charges(
        &self,
        account_id: u64,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<Charge>>, DNSimpleError> {
        let path = format!("/{}/billing/charges", account_id);

        self.client.get::<ListChargesEndpoint>(&path, options).await
    }
}
