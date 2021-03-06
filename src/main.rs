#[macro_use]
extern crate clap;
use clap::App;

extern crate clogger;
#[macro_use]
extern crate log;

use failure::Error;
use std::process;

use crate::bot::Bot;
mod bot;
mod gitlab_tag;
mod gitlab_version;

fn app() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    clogger::init();
    let mut log_verbosity = 0;

    if matches.is_present("verbose") {
        log_verbosity += 1;
    }

    if matches.is_present("debug") {
        log_verbosity += 2;
    }

    clogger::set_verbosity(log_verbosity);

    let gitlab_url = matches.value_of("gitlab_url").unwrap().to_string();
    let gitlab_token = matches.value_of("gitlab_token").unwrap().to_string();
    let rocket_chat_url = matches.value_of("rocket_chat_url").unwrap().to_string();
    let rocket_chat_token = matches.value_of("rocket_chat_token").unwrap().to_string();

    let bot = Bot {
        gitlab_url,
        gitlab_token,
        rocket_chat_url,
        rocket_chat_token,
    };
    bot.exec()?;

    Ok(())
}

fn main() {
    process::exit(match app() {
        Ok(_) => 0,
        Err(err) => {
            error!("{}", err.to_string());
            1
        }
    });
}
