use std::sync::Arc;

use scraper::{Html, Selector};
use scraper::node::Node::Text;
use ureq::serde_json::Value;
use ureq::Agent;

use crate::error::Error;
use crate::error::Error::{ParsingError, ValidationError};
use crate::models::Player;
use crate::pages::Page;

pub struct PlayerEndpoint {
    pub agent: Arc<Agent>,
}

impl PlayerEndpoint {
    pub fn get_players_by_name(
        &self,
        first_name: &str,
        last_name: &str,
    ) -> Result<Vec<Player>, Error> {
        if first_name.len() < 3 {
            return Err(ValidationError(String::from(
                "First name needs to be 3 letters long (mytischtennis requirement)",
            )));
        }

        if last_name.len() < 3 {
            return Err(ValidationError(String::from(
                "Last name needs to be 3 letters long (mytischtennis requirement)",
            )));
        }

        let response: Value = self
            .agent
            .get(&String::from(Page::PlayerSearchPage))
            .query("firstname", first_name)
            .query("lastname", last_name)
            .call()?
            .into_json()?;

        if response["availableResults"] == 0 {
            return Ok(vec![]);
        }

        let found_players: Result<Vec<Player>, Error> = response["items"]
            .as_array()
            .ok_or(ValidationError(String::from(
                "Could not get items array from player search response",
            )))?
            .iter()
            .map(|json| {
                let (qttr, ttr) = self.get_ttr_by_id(&json["personID"].to_string())?;
                Ok(Player::from(json, qttr, ttr))
            })
            .collect();

        found_players
    }

    pub fn get_ttr_by_id(&self, id: &str) -> Result<(u32, Option<u32>), Error> {
        let response = self.agent.get(&String::from(Page::TTRHistoryPage)).call()?;

        let html = Html::parse_document(&response.into_string()?);
        let selector = Selector::parse(".ttr-box>h3>span").unwrap();

        let res = html
            .select(&selector)
            .next()
            .ok_or(ParsingError("Failed to parse ttr from page".into()))
            .unwrap();

        let mut res_siblings = res.next_siblings();
        //let qttr_node= res.next_sibling().unwrap().value();
        let qttr_node = res_siblings.next().unwrap().value();

        if !qttr_node.is_text() {
            return Err(ParsingError("Node is not of expected type".into()));
        }

        let qttr_string = qttr_node.as_text().unwrap().to_string();
        let qttr = parse_ttr(&qttr_string).ok_or(ParsingError("Could not parse TTR from text".into()))?;

        let ttr_node = res_siblings.skip(3).next().unwrap().value();

        if !ttr_node.is_text() {
            return Err(ParsingError("Node is not of expected type".into()));
        }

        let ttr_string = ttr_node.as_text().unwrap().to_string();
        let ttr = parse_ttr(&ttr_string);

        Ok((qttr, ttr))
    }
}

fn parse_ttr(ttr: &str) -> Option<u32> {
    let ttr: String = ttr
        .chars()
        .skip_while(|c| !c.is_digit(10))
        .take_while(|c| c.is_digit(10))
        .collect();

    ttr.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::parse_ttr;

    #[test]
    fn test() {
        assert_eq!(Some(1400), parse_ttr("1400"));
        assert_eq!(Some(900), parse_ttr("TTR: 900"));
        assert_eq!(Some(10000), parse_ttr("QTTR: 10000 TESTSDSFDSF"));
        assert_eq!(Some(1435), parse_ttr(" 1435\n"));
        assert_eq!(None, parse_ttr(""));
    }
}
