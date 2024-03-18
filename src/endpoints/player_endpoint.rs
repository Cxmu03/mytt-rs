use std::rc::Rc;

use ureq::Agent;
use ureq::serde_json::Value;

use crate::error::Error;
use crate::error::Error::ValidationError;
use crate::models::Player;
use crate::pages::Page;

pub struct PlayerEndpoint {
    pub agent: Rc<Agent>
}

impl PlayerEndpoint {
    pub fn get_players_by_name(&self, first_name: &str, last_name: &str) -> Result<Option<Vec<Player>>, Error>{
        if first_name.len() < 3 {
            return Err(ValidationError(String::from("First name needs to be 3 letters long (mytischtennis requirement)")));
        }

        if last_name.len() < 3 {
            return Err(ValidationError(String::from("Last name needs to be 3 letters long (mytischtennis requirement)")));
        }

        let response: Value = self.agent.get(&String::from(Page::PlayerSearchPage))
            .query("firstname", first_name)
            .query("lastname", last_name)
            .call()?
            .into_json()?;

        if response["availableResults"] == 0 {
            return Ok(None);
        }

        let found_players = response["items"]
            .as_array()
            .ok_or(ValidationError(String::from("Could not get items array from player search response")))?
            .iter()
            .map(|json| {
                Player {
                    first_name: String::from(json["firstname"].as_str().unwrap()),
                    last_name: String::from(json["lastname"].as_str().unwrap()),
                    id: String::from(json["personId"].as_str().unwrap()),
                    federation: String::from(json["fedNickname"].as_str().unwrap()),
                    qttr: 0,
                    ttr: None
                }
            })
            .collect();

        Ok(Some(found_players))
    }


}