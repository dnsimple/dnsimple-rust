use std::collections::HashMap;
use crate::dnsimple::{Client, DNSimpleResponse, Endpoint, Filters, Paginate, Sort};
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
    /// use dnsimple_rust::dnsimple::new_client;
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    ///
    /// let accounts_response = client.accounts().list_accounts().unwrap();
    /// let accounts = accounts_response.data.unwrap();
    /// let first_account = accounts.first().unwrap();
    ///
    pub fn list_accounts(&self) -> Result<DNSimpleResponse<Vec<Account>>, String> {
        let filters = Filters::new(HashMap::new());
        let sort = Sort::new(String::from(""));

        self.client.get::<AccountsEndpoint>("/accounts", filters, sort, Paginate{ per_page: 0, page: 0 })
    }
}
