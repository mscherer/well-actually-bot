#![forbid(unsafe_code)]

use irc::client::prelude::*;
use futures::prelude::*;

use std::env;

#[tokio::main]
async fn main() -> irc::error::Result<()>{
    let nick = match env::var("NICK") {
        Ok(val) => val,
        Err(_e) => "testbot_guys".to_string(),
    };

    let server = match env::var("SERVER") {
        Ok(val) => val,
        Err(_e) => "chat.freenode.net".to_string(),
    };


    println!("Hello, {}!", nick);
    let config = Config {
        nickname: Some(nick),
        server: Some(server),
        ..Config::default()
    };

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let sender = client.sender();

    while let Some(message) = stream.next().await.transpose()? {
        print!("{}", message);

        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                if msg.contains(client.current_nickname()) {
                    sender.send_privmsg(target, "Hi!")?;
                }
            }
            _ => (),
        }
    }

    Ok(())
}
