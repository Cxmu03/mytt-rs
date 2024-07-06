use super::super::PlayerEndpoint;
use ureq::serde_json::Value;

#[derive(Debug)]
pub struct Player {
    pub first_name: String,
    pub last_name: String,
    pub id: String,
    pub federation: String,
    pub qttr: u32,
    pub ttr: Option<u32>,
}

impl Player {
    pub fn from(json: &Value, qttr: u32, ttr: Option<u32>) -> Self {
        Player {
            first_name: String::from(json["firstname"].as_str().unwrap()),
            last_name: String::from(json["lastname"].as_str().unwrap()),
            id: String::from(json["personId"].as_str().unwrap()),
            federation: String::from(json["fedNickname"].as_str().unwrap()),
            qttr,
            ttr,
        }
    }
}
