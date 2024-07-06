mod endpoints;
mod error;
mod models;
mod pages;

use std::sync::Arc;

use ureq::Agent;

use endpoints::*;

pub struct MyTTApi {
    pub user: UserEndpoint,
    pub player: PlayerEndpoint,
}

impl MyTTApi {
    pub fn new() -> MyTTApi {
        let agent = Arc::new(Agent::new());

        MyTTApi {
            user: UserEndpoint {
                agent: Arc::clone(&agent),
            },
            player: PlayerEndpoint {
                agent: Arc::clone(&agent),
            },
        }
    }
}
