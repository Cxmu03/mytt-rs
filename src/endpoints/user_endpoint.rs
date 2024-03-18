use std::rc::Rc;

use ureq::Agent;

use crate::error::Error;
use crate::error::Error::AuthenticationError;
use crate::pages::Page;

pub struct UserEndpoint {
    pub agent: Rc<Agent>
}

impl UserEndpoint {
    pub fn log_in(&self, username: &str, password: &str) -> Result<(), Error> {
        let json = ureq::json!({
                "userNameB": username,
                "userPassWordB": password,
                "permalogin": 1,
                "targetPage": String::from(Page::LoginRedirectPage)
            });

        let response = self.agent.post(&String::from(Page::LoginPage))
            .send_json(json)?;

        if response.get_url() != &String::from(Page::LoginRedirectPage) {
            return Err(AuthenticationError(String::from("Could not authenticate with the given credentials")));
        }

        Ok(())
    }
}