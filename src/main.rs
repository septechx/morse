use anyhow::{anyhow, bail, Result};
use std::{fmt::Display, io::stdin, process};

use crate::parse::{morse, parse};

mod lookup;
mod parse;

#[repr(u8)]
enum Mode {
    Encode = 1,
    Decode = 2,
}

impl TryFrom<u8> for Mode {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Self> {
        Ok(match value {
            1 => Mode::Encode,
            2 => Mode::Decode,
            _ => bail!("Cannot create mode from {}", value),
        })
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Mode::Encode => "Encode",
            Mode::Decode => "Decode",
        })
    }
}

fn main() -> Result<()> {
    let mut mode: Option<Mode> = None;

    loop {
        match mode {
            None => {
                mode = Some(read_mode()?);
                println!(
                    "Selected mode {}, Back (type Esc and Enter)",
                    mode.as_ref().unwrap()
                );
            }
            Some(Mode::Decode) => match process(parse)? {
                None => mode = None,
                Some(res) => println!("{res}"),
            },
            Some(Mode::Encode) => match process(morse)? {
                None => mode = None,
                Some(res) => println!("{res}"),
            },
        }
    }
}

fn read_mode() -> Result<Mode> {
    println!("Select a mode: Encode (1), Decode (2), Quit (3)");

    let mut mode = String::new();

    stdin()
        .read_line(&mut mode)
        .map_err(|_| anyhow!("Failed to read line"))?;

    let mode = mode
        .trim()
        .parse::<u8>()
        .map_err(|_| anyhow!("Input was not a number!"))?;

    match mode {
        1..=2 => (),
        _ => process::exit(0),
    }

    let mode: Mode = mode.try_into()?;

    Ok(mode)
}

fn process(fun: fn(&str) -> Result<String>) -> Result<Option<String>> {
    let mut buf = String::new();

    stdin()
        .read_line(&mut buf)
        .map_err(|_| anyhow!("Failed to read line"))?;

    let s = buf.trim();

    if s.eq_ignore_ascii_case("esc") {
        return Ok(None);
    }

    Ok(Some(fun(s)?))
}
