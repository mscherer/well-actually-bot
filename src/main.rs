#![forbid(unsafe_code)]

use futures::prelude::*;
use irc::client::prelude::*;
use radix64::STD;
use regex::Regex;
use std::env;

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let nick = match env::var("NICK") {
        Ok(val) => val,
        Err(_e) => "momopassan".to_string(),
    };

    let pass = match env::var("PASSWORD") {
        Ok(val) => Some(val),
        Err(_e) => None,
    };

    let server = Some(match env::var("SERVER") {
        Ok(val) => val,
        Err(_e) => "chat.freenode.net".to_string(),
    });

    let irc_chans = match env::var("CHANNELS") {
        Ok(val) => val
            .split(';')
            .map(String::from)
            .collect::<Vec<std::string::String>>(),
        Err(_e) => vec!["#test-misc-bot".to_string()],
    };

    let use_tls = match env::var("NO_TLS") {
        Ok(_val) => Some(false),
        Err(_e) => None,
    };

    let debug = match env::var("DEBUG") {
        Ok(_val) => true,
        Err(_e) => false,
    };

    let re = Regex::new(r"^(?i)h+(i+|ll+o+|e+y+)\s+(guy|dude)s?").unwrap();
    let answer = "https://heyguys.cc/";

    let config = Config {
        nickname: Some(nick.clone()),
        username: Some(nick.clone()),
        password: pass.clone(),
        server: server,
        channels: irc_chans,
        use_tls: use_tls,
        ..Config::default()
    };

    let mut client = Client::from_config(config).await?;
    let mut stream = client.stream()?;
    let sender = client.sender();

    // taken from https://github.com/clukawski/pybot-rs/blob/master/src/main.rs
    // https://github.com/jkhsjdhjs/chell/blob/8b752085e5dde10db9acd0ba7e7a0f18b39282a5/src/sasl.rs
    client.send_cap_req(&[Capability::Sasl])?;
    // https://ircv3.net/specs/extensions/sasl-3.1
    client.send_sasl_plain()?;
    let toencode = format!(
        "{}\0{}\0{}",
        nick,
        nick,
        pass.unwrap()
    );
    let encoded = STD.encode(&toencode);
    client.send_sasl(encoded)?;

    client.identify()?;

    while let Some(message) = stream.next().await.transpose()? {
        if debug {
            print!("debug: {}", message)
        };

        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                if re.is_match(msg) {
                    sender.send_privmsg(message.response_target().unwrap_or(target), answer)?;
                }
            }
            _ => (),
        }
    }

    Ok(())
}
