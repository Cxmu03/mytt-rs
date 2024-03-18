use mytt::MyTTApi;
use std::env;

fn main() {
    let mytt = MyTTApi::new();
    let user = env::var("MYTT_USER").unwrap();
    let password = env::var("MYTT_PASSWORD").unwrap();

    mytt.user.log_in(&user, &password).unwrap();

    let players = mytt.player.get_players_by_name("Marek", "Freunscht").unwrap().unwrap();

    dbg!(players);
}