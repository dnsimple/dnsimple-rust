use serde::{Deserialize, Serialize};

use crate::dnsimple::{Client, DNSimpleResponse, Endpoint};

/// Represents a User
///
/// See [API Documentation: identity](https://developer.dnsimple.com/v2/identity/)
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    /// The ID of the user in DNSimple
    pub id: u64,
    /// The users email
    pub email: String,
    /// When the user was created in DNSimple
    pub created_at: String,
    /// When the user was last updated in DNSimple
    pub updated_at: String,
}

/// Represents an Account
///
/// See [API Documentation: identity](https://developer.dnsimple.com/v2/identity/)
#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    /// The account ID in DNSimple
    pub id: u64,
    /// The account email
    pub email: String,
    /// The identifier of the plan the account is subscribed to
    pub plan_identifier: String,
    /// When the account was created in DNSimple
    pub created_at: String,
    /// When the account was last updated in DNSimple
    pub updated_at: String,
}

/// Represents the structure holding a User and Account structs.
///
/// See [API Documentation: identity](https://developer.dnsimple.com/v2/identity/)
#[derive(Debug, Deserialize, Serialize)]
pub struct WhoamiData{
    /// The account, if present
    pub account: Option<Account>,
    /// The user, if present
    pub user: Option<User>,
}

struct IdentityEndpoint;

impl Endpoint for IdentityEndpoint {
    type Output = WhoamiData;
}

/// The Identity Service handles the identity (whoami) endpoint of the DNSimple API.
///
/// See [API Documentation: identity](https://developer.dnsimple.com/v2/identity/)
pub struct Identity<'a> {
    pub client: &'a Client
}

impl Identity<'_> {
    /// Retrieves the details about the current authenticated entity used to access the API.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple_rust::dnsimple::{Client, new_client};
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let identity_response = client.identity().whoami().unwrap().data.unwrap();
    /// let account = identity_response.account.unwrap();
    ///
    /// ```
    pub fn whoami(&self) -> Result<DNSimpleResponse<WhoamiData>, String> {
        self.client.get::<IdentityEndpoint>("/whoami",None)
    }
}

#[cfg(test)]
mod tests {
    use crate::dnsimple::identity;

    #[test]
    fn user_fields() {
        let user = identity::User {
            id: 12,
            email: String::from("testing@dnsimple.com"),
            created_at: String::from("some_time_ago"),
            updated_at: String::from("recently"),
        };

        assert_eq!("testing@dnsimple.com", user.email)
    }

    #[test]
    fn account_fields() {
        let account = identity::Account {
            id: 14,
            email: String::from("account@dnsimple.com"),
            plan_identifier: String::from("testing_plan"),
            created_at: String::from("some_time_ago"),
            updated_at: String::from("recently"),
        };

        assert_eq!("testing_plan", account.plan_identifier)
    }
}
