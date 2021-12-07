use crate::dnsimple::{Client, DNSimpleResponse, Endpoint};
use crate::dnsimple::identity::Account;

struct AccountsEndpoint;

impl Endpoint for AccountsEndpoint {
    type Output = Vec<Account> ;
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
    /// use dnsimple::dnsimple::new_client;
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    ///
    /// let response = client.accounts().list_accounts().unwrap();
    /// let accounts = response.data.unwrap();
    /// let first_account = accounts.first().unwrap();
    /// ```
    pub fn list_accounts(&self) -> Result<DNSimpleResponse<Vec<Account>>, String> {
        self.client.get::<AccountsEndpoint>("/accounts", None)
    }
}
