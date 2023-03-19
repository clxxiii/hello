use discord::{model::Event, Discord};
use std::env;

fn main() {
    let client: Discord = Discord::from_bot_token(&env::var("DISCORD_TOKEN").expect("no token"))
        .expect("login failed");

    let (mut connection, _) = client.connect().expect("connect failed");
    loop {
        match connection.recv_event() {
            Ok(Event::Ready(_)) => {
                println!("Ready!");
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body)
            }
            Err(err) => println!("Recieved Error: {:?}", err),
        }
    }
    // This bot currently doesn't do anything except connect
}
