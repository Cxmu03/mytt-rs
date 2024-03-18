mod error;
mod pages;
mod endpoints;
mod models;

use std::rc::Rc;

use ureq::Agent;

use endpoints::*;

pub struct MyTTApi {
    pub user: UserEndpoint,
    pub player: PlayerEndpoint
}

impl MyTTApi {
    pub fn new() -> MyTTApi {
        let agent = Rc::new(Agent::new());

        MyTTApi {
            user: UserEndpoint { agent: Rc::clone(&agent) },
            player: PlayerEndpoint { agent: Rc::clone(&agent) }
        }
    }


}