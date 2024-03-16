mod error;
mod pages;

use ureq::Agent;

use error::Error;
use pages::{Page, get_page};
use crate::error::Error::AuthenticationError;

pub struct MyTTApi {
    agent: Agent
}

impl MyTTApi {
    pub fn new() -> MyTTApi {
        MyTTApi {
            agent: Agent::new()
        }
    }

    pub fn log_in(&self, username: &str, password: &str) -> Result<(), Error> {
        let json = ureq::json!({
                "userNameB": username,
                "userPassWordB": password,
                "permalogin": 1,
                "targetPage": get_page(Page::LoginRedirectPage)
            });

        let response = self.agent.post(&get_page(Page::LoginPage))
            .send_json(json)?;

        // todo: Better detection mechanism for successful login
        if response.status() != 302 {
            return Err(AuthenticationError());
        }

        Ok(())
    }
}