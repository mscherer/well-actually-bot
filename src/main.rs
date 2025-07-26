#![forbid(unsafe_code)]

use futures::prelude::*;
use irc::client::prelude::*;
use radix64::STD;
use regex::Regex;
use std::env;

#[cfg(debug_assertions)]
fn default_debug() -> bool {
    true
}

#[cfg(not(debug_assertions))]
fn default_debug() -> bool {
    false
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> irc::error::Result<()> {
    let nick = env::var("NICK").ok().or(Some("momopassan".to_string()));

    let pass = env::var("PASSWORD").ok();

    let server = env::var("SERVER")
        .ok()
        .or(Some("irc.libera.chat".to_string()));

    let channels = match env::var("CHANNELS") {
        Ok(val) => val
            .split(';')
            .map(String::from)
            .collect::<Vec<std::string::String>>(),
        Err(_) => vec!["#test-misc-bot".to_string()],
    };

    let use_tls = env::var("NO_TLS").is_ok();

    let debug = match env::var("DEBUG") {
        Ok(_) => true,
        Err(_) => default_debug(),
    };

    let re = Regex::new(r"^(?i)h+(i+|ll+o+|e+y+)\s+(guy|dude)s?").unwrap();
    let answer = "https://heyguys.cc/";

    let config = Config {
        nickname: nick.clone(),
        username: nick.clone(),
        password: pass.clone(),
        server,
        channels,
        use_tls: Some(use_tls),
        ..Config::default()
    };

    let mut client = Client::from_config(config).await?;
    let mut stream = client.stream()?;
    let sender = client.sender();

    if let Some(p) = pass {
        if let Some(n) = nick {
            // taken from https://github.com/clukawski/pybot-rs/blob/master/src/main.rs
            // https://github.com/jkhsjdhjs/chell/blob/8b752085e5dde10db9acd0ba7e7a0f18b39282a5/src/sasl.rs
            client.send_cap_req(&[Capability::Sasl])?;
            // https://ircv3.net/specs/extensions/sasl-3.1
            client.send_sasl_plain()?;
            let toencode = format!("{n}\0{n}\0{p}");
            let encoded = STD.encode(&toencode);
            client.send_sasl(encoded)?;
        }
    }
    client.identify()?;

    while let Some(message) = stream.next().await.transpose()? {
        if debug {
            print!("debug: {message}")
        };

        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            if re.is_match(msg) {
                sender.send_privmsg(message.response_target().unwrap_or(target), answer)?;
            }
        }
    }

    Ok(())
}
