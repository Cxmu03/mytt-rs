#[derive(Debug)]
pub struct Player {
    pub first_name: String,
    pub last_name: String,
    pub id: String,
    pub federation: String,
    pub qttr: u32,
    pub ttr: Option<u32>
}