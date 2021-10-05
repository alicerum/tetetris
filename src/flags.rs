use std::fmt;
use std::error::Error;
use clap::{App, Arg};

pub struct Flags {
    pub tick: u32,
}

#[derive(Debug)]
pub struct ParseError {
    details: String,
}

impl ParseError {
    pub fn new(msg: &str) -> ParseError {
        ParseError { details: String::from(msg) }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseError {}

pub fn config_flags() -> Result<Flags, ParseError> {
    let matches = App::new("tetetris")
        .version("0.1.0")
        .author("Alice Rum <wyvie@wyvie.org>")
        .arg(Arg::with_name("tick")
            .short("t")
            .long("tick-rate")
            .help("Tetris tick rate in miliseconds. Default is '300'.")
            .takes_value(true))
        .get_matches();

    let tick = matches.value_of("refresh").unwrap_or("300");
    let tick = match tick.parse() {
        Err(_) => return Err(ParseError::new("Wrong value for 'tick-rate' flag. Must be integer.")),
        Ok(v) => v,
    };

    Ok(Flags { tick })
}
