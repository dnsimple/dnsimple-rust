use serde::{Deserialize, Serialize};

use crate::dnsimple::{Client, DNSimpleResponse};
use crate::dnsimple::identity::Account;

/// Represents the Response with the accounts data
/// See [API Documentation: accounts](https://developer.dnsimple.com/v2/accounts/)
#[derive(Debug, Deserialize, Serialize)]
pub struct AccountsResponseData {
    /// A `Vec<Account>` with all the accounts
    pub data: Option<Vec<Account>>
}

/// The Accounts Service handles the accounts endpoint of the DNSimple API.
///
/// See [API Documentation: accounts](https://developer.dnsimple.com/v2/accounts/)
pub struct Accounts<'a> {
    pub client: &'a Client
}

impl Accounts<'_> {
    /// Lists the accounts the current authenticated entity has access to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple_rust::dnsimple::new_client;
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    ///
    /// let accounts_response = client.accounts().list_accounts().data;
    ///
    /// match accounts_response {
    ///     None => panic!("We should have a payload here"),
    ///     Some(accounts_data) => match accounts_data.data {
    ///         None => panic!("There should be a list of accounts here"),
    ///         Some(accounts) => {
    ///             let first_account = accounts.first().unwrap();
    ///             // do something with the account, like checking the
    ///             // plan the account is in:
    ///             //
    ///             // first_account.plan_identifier
    ///         }
    ///     }
    /// }
    pub fn list_accounts(&self) -> DNSimpleResponse<AccountsResponseData> {
        let api_response = self.client.get("/accounts");
        let raw_response = api_response.raw_http_response;
        let mut dnsimple_response = api_response.response;

        dnsimple_response.data = raw_response.into_json().unwrap();

        dnsimple_response
    }
}
