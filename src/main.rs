#![forbid(unsafe_code)]

use irc::client::prelude::*;
use futures::prelude::*;
use regex::Regex;
use std::env;

#[tokio::main]
async fn main() -> irc::error::Result<()>{
    let nick = match env::var("NICK") {
        Ok(val) => val,
        Err(_e) => "momopassan".to_string(),
    };

    let server = match env::var("SERVER") {
        Ok(val) => val,
        Err(_e) => "chat.freenode.net".to_string(),
    };

    let irc_chans = match env::var("CHANNELS") {
        Ok(val) => val.split(';').map(String::from).collect::<Vec<std::string::String>>(),
        Err(_e) => vec!["#test-misc-bot".to_string()],
    };

    let use_tls = match env::var("NO_TLS") {
        Ok(_val) => false,
        Err(_e) => true,
    };


    let re = Regex::new(r"^(?i)h+(i+|ll+o+|e+y+)\s+(guy|dude)s?").unwrap();
    let answer = "https://heyguys.cc/";

    let config = Config {
        nickname: Some(nick),
        server: Some(server),
        channels: irc_chans,
        use_tls: Some(use_tls),
        ..Config::default()
    };

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let sender = client.sender();

    while let Some(message) = stream.next().await.transpose()? {

        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                if re.is_match(msg) {
                    sender.send_privmsg(
                        message.response_target().unwrap_or(target),
                        answer)?;
                }
            }
            _ => (),
        }
    }

    Ok(())
}
