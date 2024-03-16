use mytt::MyTTApi;
use std::env;

fn main() {
    let mytt = MyTTApi::new();
    let user = env::var("MYTT_USER").unwrap();
    let password = env::var("MYTT_PASSWORD").unwrap();

    mytt.log_in(&user, &password).unwrap();
}